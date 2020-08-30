use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::action_record::{ActionCategory, ActionRecord};
use crate::schema::action_categories;
use crate::schema::action_records;
use crate::usecase::action_records::category_add::AddRecordCategoryUseCase;
use crate::usecase::action_records::record_add;
use crate::usecase::action_records::records_get::{GetRecordsUseCase, InputData};
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

impl<'a> record_add::AddRecordUseCase for ActionRecordDriver<'a> {
    fn add_record(&self, input: record_add::InputData) -> Result<ActionRecord, DataAccessError> {
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
            category,
        })
    }
}

impl<'a> GetRecordsUseCase for ActionRecordDriver<'a> {
    fn get_records(&self, input: InputData) -> Result<Vec<ActionRecord>, DataAccessError> {
        let offset = input.count * (input.page - 1);

        let record_results: Vec<RecordItem> = action_records::dsl::action_records
            .filter(action_records::dsl::user_id.eq(input.user_id))
            .limit(input.count as i64)
            .offset(offset as i64)
            .order(action_records::dsl::id.desc())
            .load::<RecordItem>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;

        let category_ids: Vec<i32> = record_results
            .iter()
            .map(|result| result.category_id)
            .filter(|id| id.is_some())
            .map(|id| id.unwrap())
            .collect();

        let categories: Vec<ActionCategory> = action_categories::dsl::action_categories
            .filter(action_categories::dsl::id.eq_any(category_ids))
            .load::<ActionCategory>(self.connection)
            .or_else(|_| Err(DataAccessError::InternalError))?;

        let results = record_results
            .iter()
            .map(|result| ActionRecord {
                user_id: result.user_id,
                id: result.id,
                start_time: result.start_time,
                end_time: result.end_time,
                info: result.info.clone(),
                category: categories
                    .iter()
                    .filter(|category| match result.category_id {
                        Some(category_id) => category.id.eq(&category_id),
                        None => false,
                    })
                    .cloned()
                    .next(),
            })
            .collect();

        Ok(results)
    }
}
