use crate::articles::usecase::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

pub trait DeleteTagDataAccess {
    fn delete(&self, target_id: i32) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: DeleteTagDataAccess,
{
    data_access.delete(input.id)
}
