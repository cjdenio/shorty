#![feature(proc_macro_hygiene, decl_macro)]

// 🏡 Local module imports
mod api;
mod attribution;
mod auth;
mod db;

use attribution::Attribution;
use db::ShortyDb;

// 👽 External create imports
#[macro_use]
extern crate rocket;

use redis::Client;
use rocket::{response::Redirect, State};
use std::{env, sync::RwLock};

pub struct ShortyState {
    db: RwLock<ShortyDb>,
}

#[get("/<name>")]
fn link(state: State<ShortyState>, name: String) -> Option<Redirect> {
    state
        .db
        .write()
        .unwrap()
        .get_link(&name)
        .map(|x| Redirect::temporary(x.url))
        .ok()
}

#[get("/")]
fn index(state: State<ShortyState>) -> Option<Redirect> {
    let mut db = state.db.write().unwrap();

    match db.get_link(&String::from("/")) {
        Ok(link) => Some(Redirect::temporary(link.url)),
        // fall back to `root`
        Err(_) => match db.get_link(&String::from("root")) {
            Ok(link) => Some(Redirect::temporary(link.url)),
            Err(_) => None,
        },
    }
}

#[catch(404)]
fn not_found() -> String {
    String::from("404 not found")
}

fn main() {
    // Make sure certain environment variables are set
    env::var("DB_URL").expect("DB_URL environment variable not set");

    let redis_client = Client::open(env::var("DB_URL").expect("Missing DB_URL env variable."))
        .expect("Error connecting to Redis");
    let db = ShortyDb::new(redis_client);

    rocket::ignite()
        .mount(
            "/",
            routes![index, link, api::add_link, api::delete_link, api::get_links],
        )
        .register(catchers![not_found])
        .manage(ShortyState {
            db: RwLock::new(db),
        })
        .attach(Attribution)
        .launch();
}
