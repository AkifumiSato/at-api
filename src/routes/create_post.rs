use crate::db::{NewPost, establish_connection, create_post};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    title: String,
    body: String,
}

pub async fn index(item: web::Json<PostJson>) -> HttpResponse {
    let connection = establish_connection();

    let new_post = NewPost {
        title: &item.title,
        body: &item.body,
    };

    match create_post(&connection, new_post) {
        Ok(post) => HttpResponse::Ok().body(format!("post title is {}", post.title)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}