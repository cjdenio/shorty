use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

use serde::Serialize;

use crate::{
    auth::ShortyToken,
    models::{ApiNewLink, Link, UpdatedLink},
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

#[get("/api/link")]
pub async fn get_links(conn: DbConn, _token: ShortyToken) -> Json<ApiResult<Vec<Link>>> {
    conn.run(|c| {
        use crate::schema::links::dsl::*;

        Json(ApiResult::from_result(links.load::<Link>(c).map(Some)))
    })
    .await
}

#[post("/api/link", data = "<link>")]
pub async fn add_link(
    conn: DbConn,
    _token: ShortyToken,
    link: Json<ApiNewLink>,
) -> Json<ApiResult<Link>> {
    // Check if URL is invalid
    if Url::parse(&link.url).is_err() {
        return Json(ApiResult {
            ok: false,
            err: Some(String::from("Invalid URL")),
            data: None,
        });
    }

    let new_link = link.0.to_new_link();

    conn.run(move |c| {
        let result = diesel::insert_into(links::table)
            .values(&new_link)
            .get_result::<Link>(c);

        Json(ApiResult::from_result(result.map(Some)))
    })
    .await
}

#[delete("/api/link/<name>")]
pub async fn delete_link(conn: DbConn, _token: ShortyToken, name: String) -> Json<ApiResult<()>> {
    conn.run(|c| {
        Json(ApiResult::from_result(
            diesel::delete(links::table.filter(links::name.eq(name)))
                .execute(c)
                .map(|_| Some(()))
                .map_err(|x| x.to_string()),
        ))
    })
    .await
}

#[patch("/api/link/<name>", data = "<link>")]
pub async fn update_link(
    conn: DbConn,
    name: String,
    link: Json<UpdatedLink>,
    _token: ShortyToken,
) -> Json<ApiResult<Link>> {
    // Make sure the URL is valid
    if let Some(x) = &link.url {
        if Url::parse(x).is_err() {
            return Json(ApiResult {
                ok: false,
                err: Some(String::from("Invalid URL")),
                data: None,
            });
        }
    }

    let link_name = if link.name == Some(String::from("")) {
        None
    } else {
        link.name.clone()
    };

    conn.run(move |c| {
        Json(ApiResult::from_result(
            diesel::update(links::table.filter(links::name.eq(name)))
                .set(&UpdatedLink {
                    name: link_name,
                    ..link.0
                })
                .get_result::<Link>(c)
                .map(Some),
        ))
    })
    .await
}
