use crate::database_utils::error::DataAccessError;
use crate::domain::entity::action_record::ActionRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub page: i32,
    pub count: i32,
}

pub trait GetRecordsUseCase {
    fn get_records(&self, input: InputData) -> Result<Vec<ActionRecord>, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<Vec<ActionRecord>, DataAccessError>
where
    T: GetRecordsUseCase,
{
    data_access.get_records(input)
}
