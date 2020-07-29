use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::db::posts::{PostTable};
use crate::db::pool::DbPool;
use crate::domain::entity::posts::Post;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    page: Option<i32>,
    count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Vec<Post>,
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Query<GetParams>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let post_table = PostTable::new(&connection);

    let page = item.page.unwrap_or_else(|| 1);
    let count = item.count.unwrap_or_else(|| 10);

    match post_table.show(count, page) {
        Ok(posts) => HttpResponse::Ok().json(Response {
            result: posts
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}