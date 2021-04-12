use rocket::{
    http::{ContentType, Status},
    response, Response,
};
use rust_embed::RustEmbed;
use std::{ffi::OsStr, io::Cursor, path::PathBuf, str};

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

#[get("/<file..>")]
pub fn admin_asset<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    AdminUi::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
            let content_type = ContentType::from_extension(ext)
                .ok_or_else(|| Status::new(400, "Could not get file content type"))?;
            response::Response::build()
                .header(content_type)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}
