use crate::usecase::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

impl InputData {
    #[cfg(test)]
    pub fn new<'a>(id: i32) -> InputData {
        InputData { id }
    }
}

pub trait DeletePostDataAccess {
    fn delete(&self, target_id: i32) -> Result<(), DataAccessError>;
}

pub fn execute<T>(post_table: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeletePostDataAccess,
{
    post_table.delete(input.id)
}
