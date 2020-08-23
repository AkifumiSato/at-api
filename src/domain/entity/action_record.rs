use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ActionRecord {
    pub id: i32,
    pub user_id: i32,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub end_time: NaiveDateTime,
    pub info: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ActionCategory {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}
