use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub published_at: NaiveDateTime,
}
