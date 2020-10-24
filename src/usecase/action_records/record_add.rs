use crate::database_utils::error::DataAccessError;
use crate::domain::entity::action_record::ActionRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub info: Option<String>,
}

pub trait AddRecordUseCase {
    fn add_record(&self, input: InputData) -> Result<ActionRecord, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<ActionRecord, DataAccessError>
where
    T: AddRecordUseCase,
{
    data_access.add_record(input)
}
