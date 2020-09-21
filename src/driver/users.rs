use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::user::User;
use crate::schema::users::{self, dsl};
use crate::usecase::users::add::CreateUserUseCase;
use crate::usecase::users::check::CheckUserUseCase;
use crate::usecase::users::delete::DeleteUserUseCase;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::driver::common::{get_user, get_registered_user};

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    uid: String,
}

impl NewUser {
    pub fn new(uid: String) -> NewUser {
        NewUser { uid }
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
    fn create(&self, uid: String) -> Result<User, DataAccessError> {
        let user = get_user(self.connection, uid.clone())?;
        if user.is_some() {
            return Err(DataAccessError::InternalErrorWithMessage(
                "Specified id is already exist!".to_string(),
            ));
        }

        let new_user = NewUser::new(uid);

        let result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> DeleteUserUseCase for UserDriver<'a> {
    fn delete(&self, uid: String) -> Result<(), DataAccessError> {
        // user registered check
        let _user = get_registered_user(self.connection, uid.clone())?;

        let result = diesel::delete(dsl::users.filter(dsl::uid.eq(uid))).execute(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

impl<'a> CheckUserUseCase for UserDriver<'a> {
    fn check_user(&self, uid: String) -> Result<Option<User>, DataAccessError> {
        // use caseの実装と共通の振る舞いは分離するため、
        // 本メソッドはget_userへの中継のみ
        get_user(self.connection, uid)
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
        let test_uid = "asdfghjkl";

        let created_posts1 = user_driver.create(test_uid.to_string()).unwrap();
        assert_eq!(created_posts1.uid, test_uid.to_string());

        let user_is_registered = user_driver.check_user(test_uid.to_string()).unwrap();
        assert!(user_is_registered.is_some());
        assert_eq!(user_is_registered.unwrap().uid, test_uid.to_string());

        let delete = user_driver.delete(test_uid.to_string());
        assert!(delete.is_ok());

        let user_is_registered = user_driver.check_user(test_uid.to_string()).unwrap();
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
        let test_uid = "asdfghjkl";

        let user_is_registered = user_driver.check_user(test_uid.to_string()).unwrap();
        match user_is_registered {
            Some(user) => user,
            None => user_driver.create(test_uid.to_string()).unwrap(),
        }
    }

    pub fn test_user_by_connection(connection: &PgConnection) -> User {
        let user_driver = UserDriver::new(&connection);
        let test_uid = "asdfghjkl";

        let user_is_registered = user_driver.check_user(test_uid.to_string()).unwrap();
        match user_is_registered {
            Some(user) => user,
            None => user_driver.create(test_uid.to_string()).unwrap(),
        }
    }
}
