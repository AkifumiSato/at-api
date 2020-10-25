use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::attendance_record::AttendanceRecord;
use crate::driver::common::get_registered_user;
use crate::schema::attendance_records;
use crate::usecase::attendance_records::update::InputData;
use crate::usecase::attendance_records::{add, search_by_user, update};
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

#[derive(AsChangeset)]
#[table_name = "attendance_records"]
pub struct UpdateRecord {
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
    break_time: Option<i32>,
}

impl UpdateRecord {
    fn new(start: Option<i64>, end: Option<i64>, break_time: Option<i32>) -> UpdateRecord {
        let start_time = start.map(|timestamp| NaiveDateTime::from_timestamp(timestamp, 0));
        let end_time = end.map(|timestamp| NaiveDateTime::from_timestamp(timestamp, 0));

        UpdateRecord {
            start_time,
            end_time,
            break_time,
        }
    }
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

impl<'a> add::AddRecordUseCase for AttendanceRecordDriver<'a> {
    fn add_record(&self, input: add::InputData) -> Result<AttendanceRecord, DataAccessError> {
        let user = get_registered_user(self.connection, input.uid)
            .or_else(|_| Err(DataAccessError::InternalError))?;
        let new_record = NewRecord {
            user_id: user.id,
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

impl<'a> search_by_user::SearchRecordsByUserUseCase for AttendanceRecordDriver<'a> {
    fn get_records(
        &self,
        input: search_by_user::InputData,
    ) -> Result<Vec<AttendanceRecord>, DataAccessError> {
        let offset = input.count * (input.page - 1);
        let user_id = get_registered_user(&self.connection, input.uid.clone())?;

        let record_results: Vec<RecordItem> = attendance_records::dsl::attendance_records
            .filter(attendance_records::dsl::user_id.eq(user_id.id))
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

impl<'a> update::UpdateRecordUseCase for AttendanceRecordDriver<'a> {
    fn update_record(&self, input: InputData) -> Result<(), DataAccessError> {
        let user = get_registered_user(self.connection, input.uid.clone())?;
        let record = attendance_records::dsl::attendance_records
            .find(input.id)
            .first::<RecordItem>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;
        if record.user_id != user.id {
            return Err(DataAccessError::InternalError);
        }

        let result = diesel::update(attendance_records::dsl::attendance_records.find(input.id))
            .set(UpdateRecord::new(
                input.start_time,
                input.end_time,
                input.break_time,
            ))
            .get_result::<RecordItem>(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

// noinspection DuplicatedCode
#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util;
    use crate::driver::users::test_utils::test_user_by_connection;
    use crate::usecase::attendance_records::add::{self, AddRecordUseCase};
    use crate::usecase::attendance_records::search_by_user::SearchRecordsByUserUseCase;
    use crate::usecase::attendance_records::update::UpdateRecordUseCase;
    use chrono::{Duration, Local};

    /// # scenario
    ///
    /// - create
    /// - get
    /// - update
    /// - get
    /// - not update
    /// - get
    #[test]
    fn attendance_driver_scenario() {
        let connection = test_util::connection_init();
        let attendance_driver = AttendanceRecordDriver::new(&connection);
        let test_user = test_user_by_connection(&connection);
        let end_time = Local::now();
        let end_time_naive = NaiveDateTime::from_timestamp(end_time.timestamp(), 0);
        let start_time = end_time - Duration::hours(8);
        let start_time_naive = NaiveDateTime::from_timestamp(start_time.timestamp(), 0);
        let break_time = 60 * 60 * 1000;

        let added_record = attendance_driver
            .add_record(add::InputData {
                uid: test_user.uid.clone(),
                start_time: start_time.timestamp(),
                end_time: end_time.timestamp(),
                break_time,
            })
            .unwrap();
        assert_eq!(added_record.user_id, test_user.id);
        assert_eq!(added_record.start_time, start_time_naive);
        assert_eq!(added_record.end_time, end_time_naive);
        assert_eq!(added_record.break_time, break_time);

        let records_by_user = attendance_driver
            .get_records(search_by_user::InputData {
                uid: get_registered_user(&attendance_driver.connection, test_user.uid.clone())
                    .unwrap()
                    .uid,
                page: 1,
                count: 1,
            })
            .unwrap();
        assert_eq!(records_by_user.len(), 1);
        let record_by_user = records_by_user.first().unwrap();
        assert_eq!(record_by_user.user_id, test_user.id);
        assert_eq!(record_by_user.start_time, start_time_naive);
        assert_eq!(record_by_user.end_time, end_time_naive);
        assert_eq!(record_by_user.break_time, break_time);

        // update
        let end_time2 = Local::now() + Duration::hours(1);
        let end_time2_naive = NaiveDateTime::from_timestamp(end_time2.timestamp(), 0);
        let start_time2 = end_time2 - Duration::hours(8);
        let start_time2_naive = NaiveDateTime::from_timestamp(start_time2.timestamp(), 0);
        let break_time2 = 60 * 60 * 1000 * 2;
        let _update_result = attendance_driver.update_record(update::InputData {
            uid: test_user.uid.clone(),
            id: record_by_user.id,
            start_time: Some(start_time2.timestamp()),
            end_time: Some(end_time2.timestamp()),
            break_time: Some(break_time2),
        });

        let records_by_user = attendance_driver
            .get_records(search_by_user::InputData {
                uid: get_registered_user(&attendance_driver.connection, test_user.uid.clone())
                    .unwrap()
                    .uid,
                page: 1,
                count: 1,
            })
            .unwrap();
        assert_eq!(records_by_user.len(), 1);
        let record_by_user = records_by_user.first().unwrap();
        assert_eq!(record_by_user.user_id, test_user.id);
        assert_eq!(record_by_user.start_time, start_time2_naive);
        assert_eq!(record_by_user.end_time, end_time2_naive);
        assert_eq!(record_by_user.break_time, break_time2);

        // not update
        let _update_result = attendance_driver.update_record(update::InputData {
            uid: test_user.uid.clone(),
            id: record_by_user.id,
            start_time: None,
            end_time: None,
            break_time: None,
        });

        // noinspection Duplicated code
        let records_by_user = attendance_driver
            .get_records(search_by_user::InputData {
                uid: get_registered_user(&attendance_driver.connection, test_user.uid.clone())
                    .unwrap()
                    .uid,
                page: 1,
                count: 1,
            })
            .unwrap();
        assert_eq!(records_by_user.len(), 1);
        let record_by_user = records_by_user.first().unwrap();
        assert_eq!(record_by_user.start_time, start_time2_naive);
        assert_eq!(record_by_user.end_time, end_time2_naive);
        assert_eq!(record_by_user.break_time, break_time2);
    }
}
