use nanoid::nanoid;
use rocket::{delete, post, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::auth::ShortyToken;
use crate::db::ShortyLink;
use crate::ShortyState;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    ok: bool,
    err: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
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

#[derive(Deserialize, Serialize)]
pub struct AddLinkParams {
    #[serde(default = "random_name")]
    name: String,
    url: String,

    #[serde(default)]
    public: bool,
}

#[post("/api/link", data = "<link>")]
pub fn add_item(
    state: State<ShortyState>,
    _token: ShortyToken,
    link: Json<AddLinkParams>,
) -> Json<ApiResult<AddLinkParams>> {
    Json(ApiResult::from_result(
        state
            .db
            .write()
            .unwrap()
            .add_link(
                &link.name,
                &ShortyLink {
                    url: link.url.clone(),
                    public: link.public,
                },
            )
            .map(|_| {
                Some(AddLinkParams {
                    url: link.url.clone(),
                    name: link.name.clone(),
                    public: link.public,
                })
            }),
    ))
}

#[delete("/api/link/<name>")]
pub fn delete_item(
    state: State<ShortyState>,
    _token: ShortyToken,
    name: String,
) -> Json<ApiResult<()>> {
    Json(ApiResult::from_result(
        state.db.write().unwrap().del_link(&name).map(|_| None),
    ))
}
