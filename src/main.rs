#![feature(proc_macro_hygiene, decl_macro)]

pub mod db;

#[macro_use]
extern crate rocket;

use std::env;

use rocket::{response::Redirect, Request};

#[get("/<name>")]
fn hello(name: String) -> Redirect {
    Redirect::temporary(format!("https://github.com/{}", name))
}

#[get("/")]
fn index() -> Option<Redirect> {
    match env::var("ROOT_URL") {
        Ok(url) => Some(Redirect::to(url)),
        Err(_) => None,
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    String::from("404 not found")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello])
        .register(catchers![not_found])
        .launch();
}
