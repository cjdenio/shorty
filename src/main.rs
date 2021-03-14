#![feature(proc_macro_hygiene, decl_macro)]

// 🏡 Local module imports
pub mod db;
use db::{ShortyDb, ShortyLink};

// 👽 External create imports
#[macro_use]
extern crate rocket;

use redis::Client;
use rocket::{response::Redirect, State};
use std::{env, sync::RwLock};

struct ShortyState {
    db: RwLock<ShortyDb>,
}

#[get("/<name>")]
fn link(state: State<ShortyState>, name: String) -> Option<Redirect> {
    match state.db.write().unwrap().get_link(&name) {
        Ok(link) => Some(Redirect::temporary(link.url)),
        Err(_) => None,
    }
}

#[get("/")]
fn index(state: State<ShortyState>) -> Option<Redirect> {
    state
        .db
        .write()
        .unwrap()
        .add_link(
            &String::from("gh"),
            &ShortyLink {
                url: String::from("https://github.com/cjdenio"),
            },
        )
        .expect("couldn't create link");

    match env::var("ROOT_URL") {
        Ok(url) => Some(Redirect::to(url)),
        Err(_) => None,
    }
}

#[catch(404)]
fn not_found() -> String {
    String::from("404 not found")
}

fn main() {
    let redis_client = Client::open(env::var("DB_URL").expect("Missing DB_URL env variable."))
        .expect("Error connecting to Redis");
    let db = ShortyDb::new(redis_client);

    rocket::ignite()
        .mount("/", routes![index, link])
        .register(catchers![not_found])
        .manage(ShortyState {
            db: RwLock::new(db),
        })
        .launch();
}
