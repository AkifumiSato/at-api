use crate::database_utils::error::DataAccessError;
use crate::domain::entity::attendance_record::AttendanceRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub break_time: i32,
}

pub trait AddRecordUseCase {
    fn add_record(&self, input: InputData) -> Result<AttendanceRecord, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<AttendanceRecord, DataAccessError>
where
    T: AddRecordUseCase,
{
    data_access.add_record(input)
}
