use crate::database_utils::pool::DbPool;
use crate::driver::post_tags::PostTagDriver;
use crate::usecase::articles::tag_create;
use crate::usecase::articles::tag_register_to_post;
use actix_web::{web, HttpResponse};

pub async fn create(
    pool: web::Data<DbPool>,
    item: web::Json<tag_create::InputData>,
) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_driver = PostTagDriver::new(&connection);

    match tag_create::execute(tags_driver, item.into_inner()) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn register(
    pool: web::Data<DbPool>,
    item: web::Json<tag_register_to_post::InputData>,
) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_driver = PostTagDriver::new(&connection);

    match tag_register_to_post::execute(tags_driver, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
