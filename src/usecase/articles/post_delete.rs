use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub uid: String,
    pub id: i32,
}

impl InputData {
    #[cfg(test)]
    pub fn new<'a>(uid: String, id: i32) -> InputData {
        InputData {
            uid,
            id,
        }
    }
}

pub trait DeletePostUseCase {
    fn delete(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeletePostUseCase,
{
    data_access.delete(input)
}
