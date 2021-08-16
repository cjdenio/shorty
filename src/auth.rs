use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use std::env;

pub struct ShortyToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ShortyToken {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = match req.headers().get_one("Authorization") {
            Some(x) => x,
            None => {
                return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")))
            }
        };

        let provided_token = match auth_header.strip_prefix("Bearer ") {
            Some(x) => x,
            None => {
                return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")))
            }
        };

        let actual_token = match env::var("TOKEN") {
            Ok(x) => x,
            Err(_) => {
                return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")))
            }
        };

        if provided_token != actual_token {
            return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")));
        }

        Outcome::Success(Self(String::from(provided_token)))
    }
}
