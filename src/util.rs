use rocket::{http::Status, Request, Response};
use rocket::{logger::error, response::Responder};
use url::Url;

pub struct Redirect(Status, String);

impl Redirect {
    pub fn temporary(url: String) -> Self {
        Self(Status::TemporaryRedirect, url)
    }
}

impl<'a> Responder<'a> for Redirect {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        if let Ok(uri) = Url::parse(&self.1.to_string()) {
            Response::build()
                .status(self.0)
                .raw_header("Location", uri.to_string())
                .ok()
        } else {
            error("Invalid URI used for redirect.");
            Err(Status::InternalServerError)
        }
    }
}
