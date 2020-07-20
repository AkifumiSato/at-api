use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::db::posts::{Post, PostTable};
use crate::db::pool::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    page: Option<i64>,
    count: i64,
}

impl PostJson {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(page: Option<i64>, count: i64) -> PostJson {
        PostJson {
            page,
            count,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Vec<Post>,
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<PostJson>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let post_table = PostTable::new(&connection);

    let page = match item.page {
        Some(x) => x,
        None => 1,
    };

    match post_table.show(item.count, page) {
        Ok(posts) => HttpResponse::Ok().json(Response {
            result: posts
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}