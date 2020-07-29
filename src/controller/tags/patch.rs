use actix_web::{web, HttpResponse};
use crate::driver::pool::DbPool;
use crate::usecase::tag_update::{self, InputData};

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<InputData>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");

    match tag_update::execute(&connection, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}