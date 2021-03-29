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

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "links"]
pub struct NewLink {
    #[serde(default = "random_name")]
    pub name: String,
    pub url: String,

    #[serde(default)]
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
