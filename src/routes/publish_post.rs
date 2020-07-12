use crate::db::{establish_connection, publish_post};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    id: i32,
}

pub async fn index(item: web::Json<PostJson>) -> HttpResponse {
    let connection = establish_connection();

    match publish_post(&connection, item.id) {
        Ok(post) => HttpResponse::Ok().body(format!("post title is {}", post.title)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}