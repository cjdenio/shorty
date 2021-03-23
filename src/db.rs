use std::collections::HashMap;

use redis::{Commands, RedisError};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct ShortyLink {
    pub url: String,

    // This is an Option<T> because I'm an idiot and some installations have this field set to `null`
    pub public: Option<bool>,
}

impl Default for ShortyLink {
    fn default() -> Self {
        Self {
            url: String::from(""),
            public: Some(false),
        }
    }
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

    pub fn del_link(&mut self, name: &String) -> Result<(), RedisError> {
        self.conn.del(format!("link:{}", name))
    }

    pub fn get_link(&mut self, name: &String) -> Result<ShortyLink, String> {
        let link_json: String = self
            .conn
            .get(format!("link:{}", name))
            .map_err(|e| e.to_string())?;

        // Fall back to URL if not JSON-parsable
        let link: ShortyLink = match serde_json::from_str(&link_json) {
            Ok(x) => x,
            Err(_) => ShortyLink {
                url: link_json,
                ..Default::default()
            },
        };

        Ok(link)
    }

    pub fn get_links(&mut self, only_public: bool) -> Result<HashMap<String, ShortyLink>, String> {
        let keys = self
            .conn
            .scan::<String>()
            .map_err(|x| x.to_string())?
            .collect::<Vec<String>>();

        let mut links: HashMap<String, ShortyLink> = HashMap::new();

        for i in keys {
            let link_name = match i.strip_prefix("link:") {
                Some(x) => x,
                None => continue,
            };

            let link = self.get_link(&link_name.to_string())?;

            if !only_public || link.public.unwrap_or(false) {
                links.insert(link_name.to_string(), link);
            }
        }

        Ok(links)
    }
}
