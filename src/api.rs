use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_contrib::json::Json;

use serde::Serialize;

use crate::{
    auth::ShortyToken,
    models::{Link, NewLink},
    DbConn,
};

use crate::schema::*;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    ok: bool,
    err: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    fn success() -> ApiResult<()> {
        ApiResult {
            ok: true,
            err: None,
            data: None,
        }
    }

    fn error<E: ToString>(err: E) -> ApiResult<()> {
        ApiResult {
            ok: false,
            err: Some(err.to_string()),
            data: None,
        }
    }

    fn from_result<E: ToString>(result: Result<Option<T>, E>) -> ApiResult<T> {
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

#[get("/api/link")]
pub fn get_links(conn: DbConn, _token: ShortyToken) -> Json<ApiResult<Vec<Link>>> {
    Json(ApiResult::from_result(
        links::table.load::<Link>(&*conn).map(|x| Some(x)),
    ))
}

#[post("/api/link", data = "<link>")]
pub fn add_link(conn: DbConn, _token: ShortyToken, link: Json<NewLink>) -> Json<ApiResult<Link>> {
    let result = diesel::insert_into(links::table)
        .values(&link.0)
        .get_result::<Link>(&*conn);

    Json(ApiResult::from_result(result.map(|x| Some(x))))
}

#[delete("/api/link/<name>")]
pub fn delete_link(conn: DbConn, _token: ShortyToken, name: String) -> Json<ApiResult<()>> {
    Json(ApiResult::from_result(
        diesel::delete(links::table.filter(links::name.eq(name)))
            .execute(&*conn)
            .map(|_| Some(()))
            .map_err(|x| x.to_string()),
    ))
}
