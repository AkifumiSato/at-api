use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::action_record::{ActionCategory, ActionRecord};
use crate::schema::action_categories;
use crate::schema::action_records;
use crate::usecase::action_records::add_category::AddRecordCategoryUseCase;
use crate::usecase::action_records::add_record;
use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

impl<'a> AddRecordCategoryUseCase for ActionRecordDriver<'a> {
    fn add_category(&self, user_id: i32, name: String) -> Result<ActionCategory, DataAccessError> {
        let result = diesel::insert_into(action_categories::table)
            .values(NewCategory::new(user_id, &name))
            .get_result::<ActionCategory>(self.connection);

        self.parse_data_access_result(result)
    }
}

#[derive(Insertable)]
#[table_name = "action_records"]
struct NewRecord {
    user_id: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    info: Option<String>,
    category_id: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
struct RecordItem {
    id: i32,
    user_id: i32,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    start_time: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    end_time: NaiveDateTime,
    info: Option<String>,
    category_id: Option<i32>,
}

impl<'a> add_record::AddRecordUseCase for ActionRecordDriver<'a> {
    fn add_record(&self, input: add_record::InputData) -> Result<ActionRecord, DataAccessError> {
        let new_record = NewRecord {
            user_id: input.user_id,
            start_time: NaiveDateTime::from_timestamp(input.start_time, 0),
            end_time: NaiveDateTime::from_timestamp(input.end_time, 0),
            info: input.info,
            category_id: input.category_id,
        };

        let record_result = diesel::insert_into(action_records::table)
            .values(new_record)
            .get_result::<RecordItem>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;

        let category = match input.category_id {
            Some(id) => action_categories::dsl::action_categories
                .find(id)
                .first::<ActionCategory>(self.connection)
                .optional()
                .or_else(|_| Err(DataAccessError::InternalError))?,
            None => None,
        };

        Ok(ActionRecord {
            id: record_result.id,
            user_id: record_result.user_id,
            start_time: record_result.start_time,
            end_time: record_result.end_time,
            info: record_result.info,
            // todo categoriesの登録実装、Entity全体の見直し
            category,
        })
    }
}
