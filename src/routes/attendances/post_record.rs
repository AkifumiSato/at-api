use crate::database_utils::pool::DbPool;
use crate::driver::attendance_records::AttendanceRecordDriver;
use crate::usecase::attendance_records::add::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostParams {
    pub uid: String,
    pub start_time: i64,
    pub end_time: i64,
    pub break_time: i32,
}

impl PostParams {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            uid: self.uid.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            break_time: self.break_time,
        }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Json<PostParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let attendance_driver = AttendanceRecordDriver::new(&connection);

    match add::execute(attendance_driver, item.to_input_data()) {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
