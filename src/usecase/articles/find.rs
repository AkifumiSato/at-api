use crate::database_utils::error::DataAccessError;
use crate::domain::entity::posts::Post;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
    pub uid: String,
}

impl InputData {
    #[cfg(test)]
    pub fn new<'a>(id: i32, uid: String) -> InputData {
        InputData { id, uid }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Option<Post>,
}

pub trait ArticleFindUseCase {
    fn find(&self, input: InputData) -> Result<Option<Post>, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<OutputData, DataAccessError>
where
    T: ArticleFindUseCase,
{
    let result = data_access.find(input)?;
    Ok(OutputData { result })
}
