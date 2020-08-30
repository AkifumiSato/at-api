use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub id: i32,
    pub name: Option<String>,
}

pub trait UpdateRecordCategoryUseCase {
    fn update_category(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: UpdateRecordCategoryUseCase,
{
    data_access.update_category(input)
}
