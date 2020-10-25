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

pub fn get_registered_user(
    connection: &PgConnection,
    uid: String,
) -> Result<User, DataAccessError> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util;
    use crate::driver::users::test_utils::test_user_by_connection;

    #[test]
    fn get_user_test() {
        let connection = test_util::connection_init();

        assert!(get_user(&connection, "asdfg".to_string()).unwrap().is_none());

        let test_user = test_user_by_connection(&connection);
        let get_user = get_user(&connection, test_user.uid.clone()).unwrap().unwrap();
        assert_eq!(get_user.uid, test_user.uid.clone());
    }

    #[test]
    fn get_registered_user_test() {
        let connection = test_util::connection_init();

        assert!(get_registered_user(&connection, "asdfg".to_string()).is_err());

        let test_user = test_user_by_connection(&connection);
        let get_user = get_registered_user(&connection, test_user.uid.clone()).unwrap();
        assert_eq!(get_user.uid, test_user.uid.clone());
    }
}
