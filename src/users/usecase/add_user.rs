use crate::database_utils::error::DataAccessError;
use serde::{Deserialize, Serialize};
use crate::users::domain::entity::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

pub trait CreateUserDataAccess {
    fn create(&self, user_id: i32) -> Result<User, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<User, DataAccessError>
where
    T: CreateUserDataAccess,
{
    data_access.create(input.id)
}
