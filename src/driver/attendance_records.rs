use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::attendance_record::{AttendanceRecord};
use crate::schema::attendance_records;
use crate::usecase::attendance_records::record_add;
use crate::usecase::attendance_records::records_get;
use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub struct AttendanceRecordDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> AttendanceRecordDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> AttendanceRecordDriver<'a> {
        AttendanceRecordDriver { connection }
    }
}

impl<'a> UseCase for AttendanceRecordDriver<'a> {}

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

impl RecordItem {
    fn to_entity(&self) -> AttendanceRecord {
        AttendanceRecord {
            id: self.id,
            user_id: self.user_id,
            start_time: self.start_time,
            end_time: self.end_time,
            break_time: self.break_time,
        }
    }
}

impl<'a> record_add::AddRecordUseCase for AttendanceRecordDriver<'a> {
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

        Ok(record_result.to_entity())
    }
}

impl<'a> records_get::GetRecordsUseCase for AttendanceRecordDriver<'a> {
    fn get_records(&self, input: records_get::InputData) -> Result<Vec<AttendanceRecord>, DataAccessError> {
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
            .map(|result| result.to_entity())
            .collect();

        Ok(results)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Local, Duration};
    use crate::database_utils::pool::test_util;
    use crate::usecase::attendance_records::record_add::{self, AddRecordUseCase};
    use crate::driver::users::test_utils::test_user_by_connection;
    use crate::usecase::attendance_records::records_get::GetRecordsUseCase;

    /// # scenario
    ///
    /// 1. create
    /// 2. get
    #[test]
    fn attendance_driver_scenario() {
        let connection = test_util::connection_init();
        let attendance_driver = AttendanceRecordDriver::new(&connection);
        let test_uid = test_user_by_connection(&connection);
        let end_time = Local::now();
        let end_time_naive = NaiveDateTime::from_timestamp(end_time.timestamp(), 0);
        let start_time = end_time - Duration::hours(8);
        let start_time_naive = NaiveDateTime::from_timestamp(start_time.timestamp(), 0);
        let break_time = 60 * 60 * 1000;

        let added_record = attendance_driver.add_record(record_add::InputData {
            user_id: test_uid.id,
            start_time: start_time.timestamp(),
            end_time: end_time.timestamp(),
            break_time,
        }).unwrap();
        assert_eq!(added_record.user_id, test_uid.id);
        assert_eq!(added_record.start_time, start_time_naive);
        assert_eq!(added_record.end_time, end_time_naive);
        assert_eq!(added_record.break_time, break_time);

        let records_by_user = attendance_driver.get_records(records_get::InputData {
            user_id: test_uid.id,
            page: 1,
            count: 1,
        }).unwrap();
        assert_eq!(records_by_user.len(), 1);
        let record_by_user = records_by_user.first().unwrap();
        assert_eq!(record_by_user.user_id, test_uid.id);
        assert_eq!(record_by_user.start_time, start_time_naive);
        assert_eq!(record_by_user.end_time, end_time_naive);
        assert_eq!(record_by_user.break_time, break_time);
    }
}
