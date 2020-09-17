use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub uid: String,
}

pub trait CreateUserUseCase {
    fn create(&self, uid: String) -> Result<User, DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<User, DataAccessError>
where
    T: CreateUserUseCase,
{
    data_access.create(input.uid)
}
