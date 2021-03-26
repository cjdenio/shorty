#![feature(proc_macro_hygiene, decl_macro)]

// 🏡 Local module imports
mod api;
mod attribution;
mod auth;
mod migrate;

mod models;
mod schema;

use std::{collections::HashMap, env};

use attribution::Attribution;
use models::Link;
use rocket_contrib::templates::Template;
use schema::*;

// 👽 External create imports
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::{expression_methods::ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use rocket::{
    config::{Environment, Value},
    Config, Rocket,
};
use rocket::{fairing::AdHoc, response::Redirect};
use serde::Serialize;

embed_migrations!();

#[database("db")]
pub struct DbConn(PgConnection);

#[get("/<name>")]
fn link(conn: DbConn, name: String) -> Option<Redirect> {
    links::table
        .filter(links::name.eq(name))
        .first::<Link>(&*conn)
        .map(|x| Redirect::temporary(x.url))
        .ok()
}

#[get("/")]
fn index(conn: DbConn) -> Option<Redirect> {
    links::table
        .filter(links::name.eq_any(vec!["/", "root"]))
        .first::<Link>(&*conn)
        .map(|x| Redirect::temporary(x.url))
        .ok()
}

#[get("/links")]
fn links(conn: DbConn) -> Template {
    #[derive(Serialize)]
    struct LinksContext {
        links: Vec<Link>,
    }

    let links: Vec<Link> = links::table
        .filter(links::public.eq(true))
        .load(&*conn)
        .unwrap();

    Template::render("links", &LinksContext { links })
}

#[catch(404)]
fn not_found() -> String {
    String::from("404 not found")
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");

    match embedded_migrations::run(&*conn) {
        Ok(_) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

fn main() -> Result<(), String> {
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
    databases.insert("db", Value::from(database_config));

    let config = Config::build(Environment::active().unwrap_or(Environment::Development))
        .port(port)
        .address("0.0.0.0")
        .extra("databases", databases)
        .finalize()
        .unwrap();

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
        .register(catchers![not_found])
        .attach(Attribution)
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database migrations", run_db_migrations))
        .attach(Template::fairing())
        .launch();

    Ok(())
}
