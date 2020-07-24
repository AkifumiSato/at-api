use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::pool::DbPool;
use crate::db::tags::{TagsTable};
use crate::model::tags::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Option<Tag>,
}

pub async fn index(
    pool: web::Data<DbPool>,
    info: web::Path<Info>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.find_by_post(info.id) {
        Ok(tags) => HttpResponse::Ok().json(Response {
            result: tags
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}