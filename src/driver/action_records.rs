use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::action_record::{ActionCategory, ActionRecord};
use crate::schema::action_records;
use crate::schema::action_categories;
use crate::usecase::action_records::add_category::AddRecordCategoryUseCase;
use crate::usecase::action_records::add_record;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "action_categories"]
struct NewCategory<'a> {
    user_id: i32,
    name: &'a str,
}

#[derive(Insertable)]
#[table_name = "action_records"]
struct NewRecord {
    user_id: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    info: Option<String>,
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

impl<'a> AddRecordCategoryUseCase for ActionRecordDriver<'a> {
    fn add_category(&self, user_id: i32, name: String) -> Result<ActionCategory, DataAccessError> {
        let result = diesel::insert_into(action_categories::table)
            .values(NewCategory::new(user_id, &name))
            .get_result::<ActionCategory>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> add_record::AddRecordUseCase for ActionRecordDriver<'a> {
    fn add_record(&self, input: add_record::InputData) -> Result<ActionRecord, DataAccessError> {
        let new_record = NewRecord {
            user_id: input.user_id,
            start_time: NaiveDateTime::from_timestamp(input.start_time, 0),
            end_time: NaiveDateTime::from_timestamp(input.end_time, 0),
            info: input.info,
        };

        let result = diesel::insert_into(action_records::table)
            .values(new_record)
            .get_result::<ActionRecord>(self.connection);

        // todo categoriesの登録実装、Entity全体の見直し

        self.parse_data_access_result(result)
    }
}
