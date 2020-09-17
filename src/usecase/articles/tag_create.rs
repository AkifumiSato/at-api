use crate::database_utils::error::DataAccessError;
use crate::domain::entity::tags::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub name: String,
    pub slug: String,
    pub user_id: i32,
}

pub trait CreateTagUseCase {
    fn create(&self, input: InputData) -> Result<Tag, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<Tag, DataAccessError>
where
    T: CreateTagUseCase,
{
    data_access.create(input)
}
