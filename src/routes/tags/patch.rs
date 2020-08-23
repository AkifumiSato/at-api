use crate::database_utils::pool::DbPool;
use crate::driver::post_tags::PostTagDriver;
use crate::usecase::articles::tag_update::{self, InputData};
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_table = PostTagDriver::new(&connection);

    match tag_update::execute(tags_table, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
