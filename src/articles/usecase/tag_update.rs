use crate::articles::usecase::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
}

pub trait UpdateTagDataAccess {
    fn update(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: UpdateTagDataAccess,
{
    data_access.update(input)
}
