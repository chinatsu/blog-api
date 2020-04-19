use super::schema::posts;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

mod datetime {
    use chrono::NaiveDateTime;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(time: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&time.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(D::Error::custom)?)
    }
}

#[derive(Deserialize, Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub content: String,
    #[serde(with = "datetime")]
    pub postdate: NaiveDateTime,
    pub hidden: bool
}

#[derive(Deserialize)]
pub struct InputPost {
    pub title: String,
    pub category: String,
    pub content: String,
    pub hidden: bool
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct InsertablePost<'a> {
    pub title: &'a str,
    pub category: &'a str,
    pub content: &'a str,
    pub hidden: bool,
}
