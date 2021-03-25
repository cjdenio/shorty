#![feature(proc_macro_hygiene, decl_macro)]

// 🏡 Local module imports
mod api;
mod attribution;
mod auth;

mod models;
mod schema;

use attribution::Attribution;
use models::Link;
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
use rocket::Rocket;
use rocket::{fairing::AdHoc, response::Redirect};

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
        .filter(links::name.eq("/"))
        .first::<Link>(&*conn)
        .map(|x| Redirect::temporary(x.url))
        .ok()
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
    rocket::ignite()
        .mount(
            "/",
            routes![index, link, api::add_link, api::delete_link, api::get_links],
        )
        .register(catchers![not_found])
        .attach(Attribution)
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database migrations", run_db_migrations))
        .launch();

    Ok(())
}
