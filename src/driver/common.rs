use crate::database_utils::error::DataAccessError;
use crate::domain::entity::user::User;
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_user(connection: &PgConnection, uid: String) -> Result<Option<User>, DataAccessError> {
    users::dsl::users
        .filter(users::dsl::uid.eq(uid.clone()))
        .first::<User>(connection)
        .optional()
        .or_else(|_| Err(DataAccessError::InternalError))
}

pub fn get_registered_user(connection: &PgConnection, uid: String) -> Result<User, DataAccessError> {
    let target_user = users::dsl::users
        .filter(users::dsl::uid.eq(uid.clone()))
        .first::<User>(connection)
        .optional()
        .or_else(|_| Err(DataAccessError::InternalError))?;

    if target_user.is_none() {
        return Err(DataAccessError::InternalErrorWithMessage(
            "User not found!".to_string(),
        ));
    } else {
        Ok(target_user.unwrap())
    }
}
