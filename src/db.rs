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
        .order(id.desc())
        .load::<Post>(connection)
}

pub fn publish_post<'a>(connection: &PgConnection, target_id: i32) -> Result<Post, diesel::result::Error> {
    use super::schema::posts::dsl::*;

    diesel::update(posts.find(target_id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() -> PgConnection {
        let db = establish_connection();
        db.begin_test_transaction().unwrap();
        db
    }

    #[test]
    fn scenario() {
        let connection = init();

        let new_post1 = NewPost {
            title: "unit test title111",
            body: "unit test body111",
        };

        let created_posts = create_post(&connection, new_post1).unwrap();
        let _published_post = publish_post(&connection, created_posts.id);

        let new_post2 = NewPost {
            title: "unit test title222",
            body: "unit test body222",
        };

        let created_posts = create_post(&connection, new_post2).unwrap();
        let _published_post = publish_post(&connection, created_posts.id);

        let posts = show_post(&connection, 2, 1).unwrap();

        let result = posts
            .iter()
            .map(|item| {
                item.title.clone()
            })
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);
    }
}