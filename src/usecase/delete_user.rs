use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

pub trait DeleteUserDataAccess {
    fn delete(&self, user_id: i32) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeleteUserDataAccess,
{
    data_access.delete(input.id)
}