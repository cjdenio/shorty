// 🏡 Local module imports
mod api;
mod auth;
mod migrate;

mod models;
mod schema;

use std::{collections::HashMap, env};

use models::Link;
use rocket::figment::value::Value;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

// 👽 External create imports
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::{Build, Config, Rocket};
use rocket_sync_db_pools::{database, diesel::PgConnection};
use serde::Serialize;

embed_migrations!();

#[database("db")]
pub struct DbConn(PgConnection);

#[derive(Serialize)]
struct LinksContext {
    links: Vec<Link>,
}

#[get("/<name>")]
async fn link(conn: DbConn, name: String) -> Option<Redirect> {
    conn.run(|c| {
        use schema::links::dsl as links;

        links::links
            .filter(links::name.eq(name))
            .first::<Link>(c)
            .map(|x| Redirect::temporary(x.url))
            .ok()
    })
    .await
}

#[get("/")]
async fn index(conn: DbConn) -> Result<Redirect, Template> {
    conn.run(|c| {
        use schema::links::dsl::*;

        links
            .filter(name.eq_any(vec!["/", "root"]))
            .first::<Link>(c)
            .map(|x| Redirect::temporary(x.url))
            .map_err(|_| {
                // Send public links page if no root link

                let fetched_links: Vec<Link> = links
                    .filter(public.eq(true))
                    .order(name.asc())
                    .load(c)
                    .unwrap();

                Template::render(
                    "links",
                    &LinksContext {
                        links: fetched_links,
                    },
                )
            })
    })
    .await
}

#[get("/links")]
async fn links(conn: DbConn) -> Template {
    conn.run(|c| {
        use schema::links::dsl::*;

        let fetched_links: Vec<Link> = links
            .filter(public.eq(true))
            .order(name.asc())
            .load(c)
            .unwrap();

        Template::render(
            "links",
            &LinksContext {
                links: fetched_links,
            },
        )
    })
    .await
}

#[catch(404)]
fn not_found() -> String {
    String::from("404 not found")
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = DbConn::get_one(&rocket).await.expect("database connection");

    conn.run(|c| match embedded_migrations::run(c) {
        Ok(_) => rocket,
        Err(_) => rocket,
    })
    .await
}

#[launch]
fn launch() -> _ {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("DB_URL"))
        .expect("DATABASE_URL env var not set");

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(8000);

    database_config.insert("url", Value::from(database_url));
    database_config.insert("pool_size", Value::from(1));
    databases.insert("db", database_config);

    let config = Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"))
        .merge(("databases", databases));

    rocket::custom(config)
        .mount(
            "/",
            routes![
                index,
                link,
                links,
                api::add_link,
                api::delete_link,
                api::get_links,
                api::update_link,
                migrate::migrate
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Database migrations", run_db_migrations))
        .attach(Template::fairing())
}
