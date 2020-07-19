use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use crate::schema::posts;
use crate::schema::posts::dsl;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
    published: bool,
}

impl<'a> NewPost<'a> {
    pub fn new(title: &'a str, body: &'a str, published: bool) -> NewPost<'a> {
        NewPost {
            title,
            body,
            published,
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl UpdatePost {
    pub fn new(title: Option<String>, body: Option<String>, published: Option<bool>) -> UpdatePost {
        UpdatePost {
            title,
            body,
            published,
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub struct PostTable<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostTable<'a> {
    pub fn new(connection: &'a PgConnection) -> PostTable<'a> {
        PostTable {
            connection,
        }
    }

    pub fn create(&self, post: NewPost) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(post)
            .get_result::<Post>(self.connection)
    }

    pub fn update(&self, target_id: i32, update_post: UpdatePost) -> Result<Post, diesel::result::Error> {
        diesel::update(dsl::posts.find(target_id))
            .set(&update_post)
            .get_result::<Post>(self.connection)
    }

    pub fn publish(&self, target_id: i32) -> Result<Post, diesel::result::Error> {
        diesel::update(dsl::posts.find(target_id))
            .set(dsl::published.eq(true))
            .get_result::<Post>(self.connection)
    }

    pub fn show(&self, count: i64, page: i64) -> Result<Vec<Post>, diesel::result::Error> {
        let offset = count * (page - 1);

        dsl::posts.filter(dsl::published.eq(true))
            .limit(count)
            .offset(offset)
            .order(dsl::id.desc())
            .load::<Post>(self.connection)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::pool::env_database_url;

    fn init() -> PgConnection {
        let database_url = env_database_url();
        let db = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        db.begin_test_transaction().unwrap();
        db
    }

    #[test]
    fn scenario() {
        let connection = init();
        let post_table = PostTable::new(&connection);

        let new_post1 = NewPost::new("unit test title111", "unit test body111", true);
        let _created_posts1 = post_table.create(new_post1).unwrap();

        let new_post2 = NewPost::new("unit test title222", "unit test body222", false);
        let created_posts2 = post_table.create(new_post2).unwrap();
        let _published_post = post_table.publish(created_posts2.id);

        let posts = post_table.show(2, 1).unwrap();

        let result = posts
            .iter()
            .map(|item| {
                item.title.clone()
            })
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);

        let update_post = UpdatePost::new(Some("update test title333".to_string()), Some("update test body333".to_string()), None);
        let _result = post_table.update(created_posts2.id, update_post);
        let posts = post_table.show(1, 1).unwrap();

        assert_eq!(posts.first().unwrap().title, "update test title333");
        assert_eq!(posts.first().unwrap().body, "update test body333");
    }
}