use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::posts;
use crate::schema::posts::dsl;
use crate::domain::entity::posts::{Post};
use crate::usecase::article_find::ArticleFindDataAccess;
use crate::usecase::error::DataAccessError;
use crate::usecase::article_list_get::ArticleListDataAccess;
use crate::driver::data_access::DataAccess;

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostUpdateAccess {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl PostUpdateAccess {
    pub fn new(title: Option<String>, body: Option<String>, published: Option<bool>) -> PostUpdateAccess {
        PostUpdateAccess {
            title,
            body,
            published,
        }
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct PostNewAccess<'a> {
    title: &'a str,
    body: &'a str,
    published: bool,
}

impl<'a> PostNewAccess<'a> {
    pub fn new(title: &'a str, body: &'a str, published: bool) -> PostNewAccess<'a> {
        PostNewAccess {
            title,
            body,
            published,
        }
    }
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

    pub fn create(&self, post: PostNewAccess) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(post)
            .get_result::<Post>(self.connection)
    }

    pub fn update(&self, target_id: i32, update_post: PostUpdateAccess) -> Result<(), diesel::result::Error> {
        let _result = diesel::update(dsl::posts.find(target_id))
            .set(&update_post)
            .get_result::<Post>(self.connection)?;
        Ok(())
    }

    pub fn delete(&self, target_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::posts.find(target_id))
            .execute(self.connection)?;
        Ok(())
    }
}

impl<'a> DataAccess for PostTable<'a> {}

impl<'a> ArticleFindDataAccess for PostTable<'a> {
    fn find(&self, id: i32) -> Result<Option<Post>, DataAccessError> {
        let result = dsl::posts.find(id)
            .first::<Post>(self.connection)
            .optional();

        self.parse_data_access_result(result)
    }
}

impl<'a> ArticleListDataAccess for PostTable<'a>{
    fn show(&self, count: i32, page: i32) -> Result<Vec<Post>, DataAccessError> {
        let offset = count * (page - 1);

        let result = dsl::posts.filter(dsl::published.eq(true))
            .limit(count as i64)
            .offset(offset as i64)
            .order(dsl::id.desc())
            .load::<Post>(self.connection);

        self.parse_data_access_result(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::driver::pool::{test_util};

    #[test]
    fn scenario() {
        let connection = test_util::connection_init();
        let post_table = PostTable::new(&connection);

        let new_post1 = PostNewAccess::new("unit test title111", "unit test body111", true);
        let created_posts1 = post_table.create(new_post1).unwrap();

        let new_post2 = PostNewAccess::new("unit test title222", "unit test body222", false);
        let created_posts2 = post_table.create(new_post2).unwrap();
        let _published_post = post_table.update(created_posts2.id, PostUpdateAccess::new(None, None, Some(true)));

        let posts = post_table.show(2, 1).unwrap();

        let result = posts
            .iter()
            .map(|item| {
                item.title.clone()
            })
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);

        let update_post = PostUpdateAccess::new(Some("update test title333".to_string()), Some("update test body333".to_string()), None);
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