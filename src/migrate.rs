use crate::api::ApiResult;
use crate::auth::ShortyToken;
use crate::models::NewLink;
use crate::DbConn;
use diesel::RunQueryDsl;
use redis::Commands;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::env;

use crate::schema::links;

#[derive(Deserialize, Debug)]
struct RedisLink {
    url: String,
    public: Option<bool>,
}

#[get("/migrate")]
pub fn migrate(conn: DbConn, _token: ShortyToken) -> Json<ApiResult<String>> {
    let redis_url = match env::var("REDIS_URL") {
        Ok(x) => x,
        Err(_) => {
            return Json(ApiResult {
                ok: false,
                err: Some(String::from("REDIS_URL not set")),
                data: None,
            })
        }
    };

    let redis_client = redis::Client::open(redis_url).expect("Error creating Redis client");
    let mut redis_conn = redis_client
        .get_connection()
        .expect("Error creating Redis connection");

    let keys: Vec<String> = redis_conn.scan().unwrap().collect();
    let mut links = Vec::<NewLink>::new();

    for key in keys {
        let link_name = match key.strip_prefix("link:") {
            Some(x) => x,
            None => continue,
        };

        let link_json: String = redis_conn.get(&key).unwrap();

        let link: RedisLink = serde_json::from_str(&link_json).unwrap_or(RedisLink {
            url: link_json,
            public: Some(false),
        });

        links.push(NewLink {
            name: String::from(link_name),
            url: link.url,
            public: link.public.unwrap_or(false),
            description: None,
        });
    }

    println!("{:?}", links);
    diesel::insert_into(links::table)
        .values(&links)
        .execute(&*conn)
        .unwrap();

    Json(ApiResult {
        ok: true,
        err: None,
        data: Some(format!(
            "Successfully migrated {} links from Redis to Postgres!",
            links.len()
        )),
    })
}
