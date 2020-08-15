use crate::articles::driver::tags::TagsTable;
use crate::articles::usecase::tag_delete::{self, InputData};
use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let tags_table = TagsTable::new(&connection);
    let input = item.into_inner();
    let id = input.id;

    match tag_delete::execute(tags_table, input) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete tag. Id is [{}]", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
