use crate::database_utils::pool::DbPool;
use crate::driver::action_records::ActionRecordDriver;
use crate::usecase::action_records::category_update::{self, InputData};
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let action_driver = ActionRecordDriver::new(&connection);

    match category_update::execute(action_driver, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
