use crate::database_utils::error::DataAccessError;
use crate::domain::entity::action_record::ActionCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub name: String,
}

pub trait CreateLogCategoryUseCase {
    fn add_category(&self, user_id: i32, name: String) -> Result<ActionCategory, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<ActionCategory, DataAccessError>
where
    T: CreateLogCategoryUseCase,
{
    data_access.add_category(input.user_id, input.name)
}
