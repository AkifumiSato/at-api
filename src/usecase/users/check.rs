use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;

pub trait CheckUserUseCase {
    fn check_user(&self, uid: String) -> Result<Option<User>, DataAccessError>;
}

pub fn execute<T>(data_access: T, uid: String) -> Result<Option<User>, DataAccessError>
where
    T: CheckUserUseCase,
{
    data_access.check_user(uid)
}
