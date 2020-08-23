use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::action_record::ActionCategory;
use crate::schema::action_categories;
use crate::usecase::action_records::add_category::CreateLogCategoryUseCase;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "action_categories"]
struct NewCategory<'a> {
    user_id: i32,
    name: &'a str,
}

impl<'a> NewCategory<'a> {
    pub fn new(user_id: i32, name: &'a str) -> NewCategory<'a> {
        NewCategory { user_id, name }
    }
}

pub struct ActionRecordDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> ActionRecordDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> ActionRecordDriver<'a> {
        ActionRecordDriver { connection }
    }
}

impl<'a> UseCase for ActionRecordDriver<'a> {}

impl<'a> CreateLogCategoryUseCase for ActionRecordDriver<'a> {
    fn add_category(&self, user_id: i32, name: String) -> Result<ActionCategory, DataAccessError> {
        let result = diesel::insert_into(action_categories::table)
            .values(NewCategory::new(user_id, &name))
            .get_result::<ActionCategory>(self.connection);

        self.parse_data_access_result(result)
    }
}
