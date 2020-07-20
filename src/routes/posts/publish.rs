use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::posts::{PostTable};
use crate::db::pool::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    id: i32,
}

impl JsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(id: i32) -> JsonBody {
        JsonBody {
            id,
        }
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<JsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let post_table = PostTable::new(&connection);

    match post_table.publish(item.id) {
        Ok(post) => HttpResponse::Ok().body(format!("post title is {}", post.title)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}