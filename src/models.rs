use super::schema::links;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub public: bool,
}

fn random_name() -> String {
    nanoid!(10)
}

#[derive(Insertable, Deserialize)]
#[table_name = "links"]
pub struct NewLink {
    #[serde(default = "random_name")]
    pub name: String,
    pub url: String,

    #[serde(default)]
    pub public: bool,
}
