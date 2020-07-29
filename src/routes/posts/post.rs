use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::driver::posts::{PostTable};
use crate::driver::pool::DbPool;
use crate::domain::entity::posts::NewPost;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    title: String,
    body: String,
    published: Option<bool>,
}

impl JsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(title: &str, body: &str, published: Option<bool>) -> JsonBody {
        JsonBody {
            title: title.to_string(),
            body: body.to_string(),
            published,
        }
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<JsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");
    let post_table = PostTable::new(&connection);

    let published = match item.published {
        Some(value) => value,
        None => false,
    };
    let new_post = NewPost::new(&item.title, &item.body, published);

    match post_table.create(new_post) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}