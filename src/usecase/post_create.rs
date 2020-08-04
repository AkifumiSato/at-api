use crate::domain::entity::posts::Post;
use crate::usecase::error::DataAccessError;
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

pub fn execute<T>(post_table: T, input: InputData) -> Result<Post, DataAccessError>
where
    T: CreatePostDataAccess,
{
    post_table.create(input)
}
