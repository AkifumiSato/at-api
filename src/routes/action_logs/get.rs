use crate::database_utils::pool::DbPool;
use crate::driver::attendance_records::ActionRecordDriver;
use crate::usecase::attendance_records::records_get::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub user_id: i32,
    pub page: Option<i32>,
    pub count: Option<i32>,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        let page = self.page.unwrap_or_else(|| 1);
        let count = self.count.unwrap_or_else(|| 10);

        InputData {
            user_id: self.user_id,
            page,
            count,
        }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let action_driver = ActionRecordDriver::new(&connection);

    match records_get::execute(action_driver, item.to_input_data()) {
        Ok(action_records) => HttpResponse::Created().json(action_records),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
