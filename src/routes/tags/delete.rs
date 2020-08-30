use crate::database_utils::pool::DbPool;
use crate::driver::post_tags::PostTagDriver;
use crate::usecase::articles::tag_delete::{self, InputData};
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_driver = PostTagDriver::new(&connection);
    let input = item.into_inner();
    let id = input.id;

    match tag_delete::execute(tags_driver, input) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete tag. Id is [{}]", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
