use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub uid: String,
}

pub trait DeleteUserUseCase {
    fn delete(&self, uid: String) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeleteUserUseCase,
{
    data_access.delete(input.uid)
}
