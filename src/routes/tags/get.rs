use crate::database_utils::pool::DbPool;
use crate::driver::post_tags::PostTagDriver;
use crate::usecase::articles::tag_all_get;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_table = PostTagDriver::new(&connection);

    match tag_all_get::execute(tags_table) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
