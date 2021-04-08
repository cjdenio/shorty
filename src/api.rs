use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;

use serde::Serialize;

use crate::{
    auth::ShortyToken,
    models::{Link, NewLink, UpdatedLink},
    DbConn,
};

use url::Url;

use crate::schema::*;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    pub ok: bool,
    pub err: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn success() -> ApiResult<()> {
        ApiResult {
            ok: true,
            err: None,
            data: None,
        }
    }

    pub fn error<E: ToString>(err: E) -> ApiResult<()> {
        ApiResult {
            ok: false,
            err: Some(err.to_string()),
            data: None,
        }
    }

    pub fn from_result<E: ToString>(result: Result<Option<T>, E>) -> ApiResult<T> {
        match result {
            Ok(r) => ApiResult {
                ok: true,
                err: None,
                data: r,
            },
            Err(x) => ApiResult {
                ok: false,
                err: Some(x.to_string()),
                data: None,
            },
        }
    }
}

#[get("/link")]
pub fn get_links(conn: DbConn, _token: ShortyToken) -> Json<ApiResult<Vec<Link>>> {
    Json(ApiResult::from_result(
        links::table.load::<Link>(&*conn).map(|x| Some(x)),
    ))
}

#[post("/link", data = "<link>")]
pub fn add_link(conn: DbConn, _token: ShortyToken, link: Json<NewLink>) -> Json<ApiResult<Link>> {
    // Check if URL is invalid
    if let Err(_) = Url::parse(&link.url) {
        return Json(ApiResult {
            ok: false,
            err: Some(String::from("Invalid URL")),
            data: None,
        });
    }

    let result = diesel::insert_into(links::table)
        .values(&link.0)
        .get_result::<Link>(&*conn);

    Json(ApiResult::from_result(result.map(|x| Some(x))))
}

#[delete("/link/<name>")]
pub fn delete_link(conn: DbConn, _token: ShortyToken, name: String) -> Json<ApiResult<()>> {
    Json(ApiResult::from_result(
        diesel::delete(links::table.filter(links::name.eq(name)))
            .execute(&*conn)
            .map(|_| Some(()))
            .map_err(|x| x.to_string()),
    ))
}

#[patch("/link/<name>", data = "<link>")]
pub fn update_link(
    conn: DbConn,
    name: String,
    link: Json<UpdatedLink>,
    _token: ShortyToken,
) -> Json<ApiResult<Link>> {
    // Make sure the URL is valid
    if let Some(x) = &link.url {
        if let Err(_) = Url::parse(x) {
            return Json(ApiResult {
                ok: false,
                err: Some(String::from("Invalid URL")),
                data: None,
            });
        }
    }

    Json(ApiResult::from_result(
        diesel::update(links::table.filter(links::name.eq(name)))
            .set(&link.0)
            .get_result::<Link>(&*conn)
            .map(|x| Some(x)),
    ))
}

#[get("/test_auth")]
pub fn test_auth(_token: ShortyToken) -> () {}
