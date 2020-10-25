use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceRecord {
    pub id: i32,
    pub user_id: i32,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub end_time: NaiveDateTime,
    pub break_time: i32,
}
