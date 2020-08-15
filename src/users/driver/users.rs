use super::super::usecase::add_user;
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::database_utils::error::{DataAccess, DataAccessError};
use crate::users::domain::entity::user::User;
use crate::users::usecase::add_user::CreateUserDataAccess;

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser {
    id: i32,
}

impl NewUser {
    pub fn new(id: i32) -> NewUser {
        NewUser {
            id,
        }
    }
}

pub struct UserTable<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserTable<'a> {
    pub fn new(connection: &'a PgConnection) -> UserTable<'a> {
        UserTable { connection }
    }
}

impl<'a> DataAccess for UserTable<'a> {}

impl<'a> CreateUserDataAccess for UserTable<'a> {
    fn create(&self, input: add_user::InputData) -> Result<User, DataAccessError> {
        let new_user = NewUser::new(input.id);

        let result = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(self.connection);

        self.parse_data_access_result(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util;

    #[test]
    fn scenario() {
        let connection = test_util::connection_init();
        let user_table = UserTable::new(&connection);

        let new_input1 = add_user::InputData {
            id: 9999,
        };
        let created_posts1 = user_table.create(new_input1).unwrap();

        assert_eq!(created_posts1.id, 9999);
    }
}
