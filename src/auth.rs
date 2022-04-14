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

        let actual_token  = if let Ok(token) = env::var("TOKEN") {
            token
        } else if let Ok(token_file) = env::var("TOKEN_FILE") {
            let read_result = std::fs::read_to_string(token_file);
            if read_result.is_err() {
                return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")));
            }
            read_result.expect("")
        } else {
                return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")));
        };

        if provided_token != actual_token {
            return Outcome::Failure((Status::Unauthorized, String::from("Invalid API token.")));
        }

        Outcome::Success(Self(String::from(provided_token)))
    }
}
