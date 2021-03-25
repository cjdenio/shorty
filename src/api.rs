use diesel::RunQueryDsl;
use nanoid::nanoid;
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use crate::{auth::ShortyToken, models::Link, DbConn};

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

fn random_name() -> String {
    nanoid!(10)
}

#[get("/api/link")]
pub fn get_links(conn: DbConn, _token: ShortyToken) -> Json<ApiResult<Vec<Link>>> {
    Json(ApiResult::from_result(
        links::table.load::<Link>(&*conn).map(|x| Some(x)),
    ))
}

#[derive(Deserialize, Serialize)]
pub struct AddLinkParams {
    #[serde(default = "random_name")]
    name: String,
    url: String,

    #[serde(default)]
    public: bool,
}

#[post("/api/link", data = "<link>")]
pub fn add_link(_token: ShortyToken, link: Json<AddLinkParams>) -> Json<ApiResult<()>> {
    Json(ApiResult::<()>::success())
}

#[delete("/api/link/<name>")]
pub fn delete_link(_token: ShortyToken, name: String) -> Json<ApiResult<()>> {
    Json(ApiResult::<()>::success())
}
