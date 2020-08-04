use crate::driver::pool::DbPool;
use crate::usecase::tag_all_get;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");

    match tag_all_get::execute(&connection) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
