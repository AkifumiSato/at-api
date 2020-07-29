use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::driver::tags::{TagsTable};
use crate::driver::pool::DbPool;
use crate::domain::entity::tags::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Vec<Tag>,
}

pub async fn index(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.all_tags() {
        Ok(tags) => HttpResponse::Ok().json(Response {
            result: tags
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}