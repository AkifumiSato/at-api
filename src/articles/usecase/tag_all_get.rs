use crate::articles::domain::entity::tags::Tag;
use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<Tag>,
}

pub trait TagAllGetDataAccess {
    fn all_tags(&self) -> Result<Vec<Tag>, DataAccessError>;
}

pub fn execute<T>(data_access: T) -> Result<OutputData, DataAccessError>
where
    T: TagAllGetDataAccess,
{
    let result = data_access.all_tags()?;
    Ok(OutputData { result })
}
