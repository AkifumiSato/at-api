use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

pub trait CreateUserUseCase {
    fn create(&self, user_id: i32) -> Result<User, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<User, DataAccessError>
where
    T: CreateUserUseCase,
{
    data_access.create(input.id)
}
