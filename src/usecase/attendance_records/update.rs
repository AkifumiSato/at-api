use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub uid: String,
    pub id: i32,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub break_time: Option<i32>,
}

pub trait UpdateRecordUseCase {
    fn update_record(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: UpdateRecordUseCase,
{
    data_access.update_record(input)
}
