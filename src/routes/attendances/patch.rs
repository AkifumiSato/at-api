use crate::database_utils::pool::DbPool;
use crate::driver::attendance_records::AttendanceRecordDriver;
use crate::usecase::attendance_records::update::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    pub uid: String,
    pub id: i32,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub break_time: Option<i32>,
}

impl JsonBody {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            uid: self.uid.clone(),
            id: self.id,
            start_time: self.start_time,
            end_time: self.end_time,
            break_time: self.break_time,
        }
    }
}

pub async fn route(pool: web::Data<DbPool>, item: web::Json<JsonBody>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let attendance_driver = AttendanceRecordDriver::new(&connection);

    match update::execute(attendance_driver, item.to_input_data()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
