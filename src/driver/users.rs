use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::user::User;
use crate::schema::users::{self, dsl};
use crate::usecase::users::add::CreateUserUseCase;
use crate::usecase::users::check::CheckUserUseCase;
use crate::usecase::users::delete::DeleteUserUseCase;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    id: i32,
}

impl NewUser {
    pub fn new(id: i32) -> NewUser {
        NewUser { id }
    }
}

pub struct UserDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> UserDriver<'a> {
        UserDriver { connection }
    }
}

impl<'a> UseCase for UserDriver<'a> {}

impl<'a> CreateUserUseCase for UserDriver<'a> {
    fn create(&self, user_id: i32) -> Result<User, DataAccessError> {
        let new_user = NewUser::new(user_id);

        let result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> DeleteUserUseCase for UserDriver<'a> {
    fn delete(&self, user_id: i32) -> Result<(), DataAccessError> {
        let result = diesel::delete(dsl::users.find(user_id)).execute(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

impl<'a> CheckUserUseCase for UserDriver<'a> {
    fn is_registered(&self, user_id: i32) -> Result<Option<User>, DataAccessError> {
        let result = dsl::users
            .find(user_id)
            .first::<User>(self.connection)
            .optional();

        self.parse_data_access_result(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util;

    #[test]
    fn user_driver_scenario() {
        let connection = test_util::connection_init();
        let user_driver = UserDriver::new(&connection);

        let created_posts1 = user_driver.create(9999).unwrap();
        assert_eq!(created_posts1.id, 9999);

        let user_is_registered = user_driver.is_registered(9999).unwrap();
        assert!(user_is_registered.is_some());
        assert_eq!(user_is_registered.unwrap().id, 9999);

        let delete = user_driver.delete(9999);
        assert!(delete.is_ok());

        let user_is_registered = user_driver.is_registered(9999).unwrap();
        assert!(user_is_registered.is_none());
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::database_utils::pool::DbPool;

    pub fn test_user_by_pool(pool: DbPool) -> User {
        let connection = pool
            .get()
            .expect("couldn't get driver connection from pool");
        let user_driver = UserDriver::new(&connection);

        let user_is_registered = user_driver.is_registered(9999).unwrap();
        match user_is_registered {
            Some(user) => user,
            None => user_driver.create(9999).unwrap(),
        }
    }

    pub fn test_user_by_connection(connection: &PgConnection) -> User {
        let user_driver = UserDriver::new(&connection);

        let user_is_registered = user_driver.is_registered(9999).unwrap();
        match user_is_registered {
            Some(user) => user,
            None => user_driver.create(9999).unwrap(),
        }
    }
}
