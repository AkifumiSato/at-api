use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
}

pub trait UpdateTagUseCase {
    fn update(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: UpdateTagUseCase,
{
    data_access.update(input)
}
