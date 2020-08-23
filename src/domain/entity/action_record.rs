use chrono::naive::serde::ts_seconds::serialize;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct ActionRecord {
    pub id: i32,
    pub user_id: i32,
    #[serde(serialize_with = "serialize")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    pub end_time: NaiveDateTime,
    pub info: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ActionCategory {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}
