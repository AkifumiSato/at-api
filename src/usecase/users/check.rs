use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;

pub trait CheckUserUseCase {
    fn is_registered(&self, user_id: i32) -> Result<Option<User>, DataAccessError>;
}

pub fn execute<T>(data_access: T, id: i32) -> Result<Option<User>, DataAccessError>
where
    T: CheckUserUseCase,
{
    data_access.is_registered(id)
}
