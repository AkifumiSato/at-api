use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub start: i64,
    pub end: i64,
    pub info: String,
}

pub trait CreateUserDataAccess {
    fn create(&self, user_id: i32) -> Result<User, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<User, DataAccessError>
where
    T: CreateUserDataAccess,
{
    data_access.create(input.user_id)
}
