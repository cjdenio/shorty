use rocket::{fairing::Fairing, fairing::Info, fairing::Kind, http::Header};

use rocket::{Request, Response};

pub struct Attribution;

impl Fairing for Attribution {
    fn info(&self) -> Info {
        Info {
            name: "Attribution",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        response.set_header(Header::new(
            "X-Powered-By",
            "shorty (https://github.com/cjdenio/shorty)",
        ));
    }
}
