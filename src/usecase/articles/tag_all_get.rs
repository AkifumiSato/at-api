use crate::database_utils::error::DataAccessError;
use crate::domain::entity::tags::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<Tag>,
}

pub trait TagAllGetUseCase {
    fn all_tags(&self) -> Result<Vec<Tag>, DataAccessError>;
}

pub fn execute<T>(data_access: T) -> Result<OutputData, DataAccessError>
where
    T: TagAllGetUseCase,
{
    let result = data_access.all_tags()?;
    Ok(OutputData { result })
}
