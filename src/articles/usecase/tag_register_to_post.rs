use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub post_id: i32,
    pub tag_id: i32,
}

pub trait RegisterTagPostDataAccess {
    fn register_tag_post(&self, post_id: i32, tag_id: i32) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: RegisterTagPostDataAccess,
{
    data_access.register_tag_post(input.post_id, input.tag_id)
}
