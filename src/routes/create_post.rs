use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::posts;
use crate::db::{Post, NewPost, establish_connection};
use crate::diesel;

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

    match diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(&connection) {
        Ok(post) => HttpResponse::Ok().body(format!("post title is {}", post.title)),
        Err(_e) => HttpResponse::Ok().body("Error saving new post"),
    }
}