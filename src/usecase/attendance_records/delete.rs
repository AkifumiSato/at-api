use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub uid: String,
    pub id: i32,
}

pub trait DeleteRecordUseCase {
    fn delete_record(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeleteRecordUseCase,
{
    data_access.delete_record(input)
}
