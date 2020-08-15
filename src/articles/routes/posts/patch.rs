use crate::articles::driver::posts::PostTable;
use crate::articles::usecase::post_update::{self, InputData};
use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_table = PostTable::new(&connection);

    match post_update::execute(post_table, item.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
