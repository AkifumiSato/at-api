use actix_web::{web, HttpResponse};
use crate::driver::pool::DbPool;
use crate::usecase::article_find::{self, InputData};

pub async fn index(
    pool: web::Data<DbPool>,
    info: web::Path<InputData>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");

    match article_find::execute(&connection, info.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}