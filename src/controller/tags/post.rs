use actix_web::{web, HttpResponse};
use crate::driver::pool::DbPool;
use crate::usecase::tag_create;
use crate::usecase::tag_register_to_post;

pub async fn create(
    pool: web::Data<DbPool>,
    item: web::Json<tag_create::InputData>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");

    match tag_create::execute(&connection, item.into_inner()) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn register(
    pool: web::Data<DbPool>,
    item: web::Json<tag_register_to_post::InputData>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");

    match tag_register_to_post::execute(&connection, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}