use rocket::{delete, get, post, request::LenientForm, State};
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::auth::ShortyToken;
use crate::db::ShortyLink;
use crate::ShortyState;

#[derive(Serialize)]
pub struct ApiResult {
    ok: bool,
    err: Option<String>,
}

impl ApiResult {
    fn from_result<R, E: ToString>(result: Result<R, E>) -> ApiResult {
        match result {
            Ok(_) => ApiResult {
                ok: true,
                err: None,
            },
            Err(x) => ApiResult {
                ok: false,
                err: Some(x.to_string()),
            },
        }
    }
}

#[post("/api/link/<name>", data = "<link>")]
pub fn add_item(
    state: State<ShortyState>,
    _token: ShortyToken,
    name: String,
    link: LenientForm<ShortyLink>,
) -> Json<ApiResult> {
    Json(ApiResult::from_result(
        state.db.write().unwrap().add_link(&name, &link),
    ))
}

#[delete("/api/link/<name>")]
pub fn delete_item(
    state: State<ShortyState>,
    _token: ShortyToken,
    name: String,
) -> Json<ApiResult> {
    Json(ApiResult::from_result(
        state.db.write().unwrap().del_link(&name),
    ))
}
