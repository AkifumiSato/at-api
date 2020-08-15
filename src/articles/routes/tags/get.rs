use crate::articles::driver::tags::TagsTable;
use crate::articles::usecase::tag_all_get;
use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_table = TagsTable::new(&connection);

    match tag_all_get::execute(tags_table) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
