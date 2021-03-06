use crate::database_utils::pool::DbPool;
use crate::driver::attendance_records::AttendanceRecordDriver;
use crate::usecase::attendance_records::search_by_user::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub uid: String,
    pub page: Option<i32>,
    pub count: Option<i32>,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        let page = self.page.unwrap_or_else(|| 1);
        let count = self.count.unwrap_or_else(|| 10);

        InputData {
            uid: self.uid.clone(),
            page,
            count,
        }
    }
}

pub async fn route(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let attendance_driver = AttendanceRecordDriver::new(&connection);

    match search_by_user::execute(attendance_driver, item.to_input_data()) {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
