use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use super::schema::posts;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn create_post<'a>(connection: &PgConnection, post: NewPost) -> Result<Post, diesel::result::Error> {
    diesel::insert_into(posts::table)
        .values(post)
        .get_result::<Post>(connection)
}

pub fn show_post<'a>(connection: &PgConnection, count: i64, page: i64) -> Result<Vec<Post>, diesel::result::Error> {
    use super::schema::posts::dsl::*;

    let offset = count * (page - 1);

    posts.filter(published.eq(true))
        .limit(count)
        .offset(offset)
        .load::<Post>(connection)
}

pub fn publish_post<'a>(connection: &PgConnection, target_id: i32) -> Result<Post, diesel::result::Error> {
    use super::schema::posts::dsl::*;

    diesel::update(posts.find(target_id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
}