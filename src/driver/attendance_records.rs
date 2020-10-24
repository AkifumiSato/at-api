use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::attendance_record::{AttendanceRecord};
use crate::schema::attendance_records;
use crate::usecase::attendance_records::record_add;
use crate::usecase::attendance_records::records_get::{GetRecordsUseCase, InputData};
use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ActionRecordDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> ActionRecordDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> ActionRecordDriver<'a> {
        ActionRecordDriver { connection }
    }
}

impl<'a> UseCase for ActionRecordDriver<'a> {}

#[derive(Insertable)]
#[table_name = "attendance_records"]
struct NewRecord {
    user_id: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    break_time: i32,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
struct RecordItem {
    id: i32,
    user_id: i32,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    end_time: NaiveDateTime,
    break_time: i32,
}

impl<'a> record_add::AddRecordUseCase for ActionRecordDriver<'a> {
    fn add_record(&self, input: record_add::InputData) -> Result<AttendanceRecord, DataAccessError> {
        let new_record = NewRecord {
            user_id: input.user_id,
            start_time: NaiveDateTime::from_timestamp(input.start_time, 0),
            end_time: NaiveDateTime::from_timestamp(input.end_time, 0),
            break_time: input.break_time,
        };

        let record_result = diesel::insert_into(attendance_records::table)
            .values(new_record)
            .get_result::<RecordItem>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;

        Ok(AttendanceRecord {
            id: record_result.id,
            user_id: record_result.user_id,
            start_time: record_result.start_time,
            end_time: record_result.end_time,
            break_time: record_result.break_time,
        })
    }
}

impl<'a> GetRecordsUseCase for ActionRecordDriver<'a> {
    fn get_records(&self, input: InputData) -> Result<Vec<AttendanceRecord>, DataAccessError> {
        let offset = input.count * (input.page - 1);

        let record_results: Vec<RecordItem> = attendance_records::dsl::attendance_records
            .filter(attendance_records::dsl::user_id.eq(input.user_id))
            .limit(input.count as i64)
            .offset(offset as i64)
            .order(attendance_records::dsl::id.desc())
            .load::<RecordItem>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;

        let results = record_results
            .iter()
            .map(|result| AttendanceRecord {
                user_id: result.user_id,
                id: result.id,
                start_time: result.start_time,
                end_time: result.end_time,
                break_time: result.break_time,
            })
            .collect();

        Ok(results)
    }
}
