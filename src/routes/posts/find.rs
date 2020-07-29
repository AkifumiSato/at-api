use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::posts::{PostTable};
use crate::db::pool::DbPool;
use crate::domain::entity::posts::Post;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Option<Post>,
}

pub async fn index(
    pool: web::Data<DbPool>,
    info: web::Path<Info>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let post_table = PostTable::new(&connection);

    match post_table.find(info.id) {
        Ok(post) => HttpResponse::Ok().json(Response {
            result: post
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}