use super::schema::links;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub public: bool,
    pub description: Option<String>,
}

fn random_name() -> String {
    nanoid!(5)
}

#[derive(Deserialize, Clone)]
pub struct ApiNewLink {
    pub name: Option<String>,
    pub url: String,
    pub public: Option<bool>,
    pub description: Option<String>,
}

impl ApiNewLink {
    pub fn to_new_link(self) -> NewLink {
        NewLink {
            name: match self.name.as_deref() {
                Some("") | None => random_name(),
                Some(x) => String::from(x),
            },
            description: self.description,
            public: self.public.unwrap_or(false),
            url: self.url,
        }
    }
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "links"]
pub struct NewLink {
    pub name: String,
    pub url: String,
    pub public: bool,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, AsChangeset)]
#[table_name = "links"]
pub struct UpdatedLink {
    pub name: Option<String>,
    pub url: Option<String>,
    pub public: Option<bool>,
    pub description: Option<String>,
}
