use crate::articles::domain::entity::posts::Post;
use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub trait CreatePostDataAccess {
    fn create(&self, input: InputData) -> Result<Post, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<Post, DataAccessError>
where
    T: CreatePostDataAccess,
{
    data_access.create(input)
}
