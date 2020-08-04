use crate::domain::entity::tags::Tag;
use crate::usecase::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub name: String,
    pub slug: String,
}

pub trait CreateTagDataAccess {
    fn create(&self, input: InputData) -> Result<Tag, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<Tag, DataAccessError>
where
    T: CreateTagDataAccess,
{
    data_access.create(input)
}
