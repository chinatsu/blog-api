use super::schema::newsitems;
use serde::{Serialize};

#[derive(Serialize, Queryable)]
pub struct NewsItem {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "newsitems"]
pub struct InsertableNewsItem<'a> {
    pub title: &'a str,
}
