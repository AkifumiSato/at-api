use chrono::naive::serde::ts_seconds::{serialize};
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(serialize_with = "serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    pub published_at: NaiveDateTime,
}
