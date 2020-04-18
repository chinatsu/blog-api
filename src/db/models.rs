use super::schema::posts;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost<'a> {
    pub title: &'a str,
    pub category: &'a str,
    pub content: &'a str,
}
