use super::schema::links;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub public: bool,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub name: String,
    pub url: String,
    pub public: bool,
}
