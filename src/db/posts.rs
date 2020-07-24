use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::posts;
use crate::schema::posts::dsl;
use crate::model::posts::{NewPost, UpdatePost, Post};

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

    pub fn update(&self, target_id: i32, update_post: UpdatePost) -> Result<(), diesel::result::Error> {
        let _result = diesel::update(dsl::posts.find(target_id))
            .set(&update_post)
            .get_result::<Post>(self.connection)?;
        Ok(())
    }

    pub fn show(&self, count: i32, page: i32) -> Result<Vec<Post>, diesel::result::Error> {
        let offset = count * (page - 1);

        dsl::posts.filter(dsl::published.eq(true))
            .limit(count as i64)
            .offset(offset as i64)
            .order(dsl::id.desc())
            .load::<Post>(self.connection)
    }

    pub fn find(&self, id: i32) -> Result<Option<Post>, diesel::result::Error> {
        dsl::posts.find(id)
            .first::<Post>(self.connection)
            .optional()
    }

    pub fn delete(&self, target_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::posts.find(target_id))
            .execute(self.connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::db::pool::{test_util};

    #[test]
    fn scenario() {
        let connection = test_util::connection_init();
        let post_table = PostTable::new(&connection);

        let new_post1 = NewPost::new("unit test title111", "unit test body111", true);
        let created_posts1 = post_table.create(new_post1).unwrap();

        let new_post2 = NewPost::new("unit test title222", "unit test body222", false);
        let created_posts2 = post_table.create(new_post2).unwrap();
        let _published_post = post_table.update(created_posts2.id, UpdatePost::new(None, None, Some(true)));

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

        let _result = post_table.delete(created_posts2.id);
        let posts = post_table.show(1, 1).unwrap();
        assert_ne!(posts.first().unwrap().title, "update test title333");

        let result = post_table.find(created_posts1.id).unwrap().unwrap();
        assert_eq!(result.title, "unit test title111");

        let result = post_table.find(created_posts2.id).unwrap();
        assert!(result.is_none());
    }
}