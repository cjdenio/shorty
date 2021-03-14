use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortyLink {
    pub url: String,
}

pub struct ShortyDb {
    conn: redis::Client,
}

impl ShortyDb {
    pub fn new(conn: redis::Client) -> Self {
        ShortyDb { conn }
    }

    pub fn add_link(&mut self, name: &String, link: &ShortyLink) -> Result<(), RedisError> {
        let link_json = serde_json::to_string(&link).unwrap();

        let _: () = self.conn.set(format!("link:{}", name), link_json)?;

        Ok(())
    }

    pub fn get_link(&mut self, name: &String) -> Result<ShortyLink, String> {
        let link_json: String = self
            .conn
            .get(format!("link:{}", name))
            .map_err(|e| e.to_string())?;

        // Fall back to URL if not JSON-parsable
        let link: ShortyLink = match serde_json::from_str(&link_json) {
            Ok(x) => x,
            Err(_) => ShortyLink { url: link_json },
        };

        Ok(link)
    }
}
