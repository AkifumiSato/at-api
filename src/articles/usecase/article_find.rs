use crate::articles::domain::entity::posts::Post;
use crate::articles::usecase::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Option<Post>,
}

pub trait ArticleFindDataAccess {
    fn find(&self, id: i32) -> Result<Option<Post>, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<OutputData, DataAccessError>
where
    T: ArticleFindDataAccess,
{
    let result = data_access.find(input.id)?;
    Ok(OutputData { result })
}
