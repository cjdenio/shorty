use rocket::{http::Status, response, Response};
use rust_embed::RustEmbed;
use std::{io::Cursor, str};

#[derive(RustEmbed)]
#[folder = "admin/dist"]
struct AdminUi;

#[get("/")]
pub fn admin_index<'a>() -> response::Result<'a> {
    AdminUi::get("index.html").map_or_else(
        || Err(Status::NotFound),
        |d| Response::build().sized_body(Cursor::new(d)).ok(),
    )
}
