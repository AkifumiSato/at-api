use crate::database_utils::pool::DbPool;
use crate::driver::posts::PostDriver;
use crate::usecase::articles::post_delete::{self, InputData};
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_driver = PostDriver::new(&connection);
    let input = item.into_inner();
    let id = input.id;

    match post_delete::execute(post_driver, input) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete post [{}]", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
