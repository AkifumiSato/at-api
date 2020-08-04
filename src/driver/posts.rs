use crate::domain::entity::posts::Post;
use crate::driver::data_access::DataAccess;
use crate::schema::posts;
use crate::schema::posts::dsl;
use crate::usecase::article_find::ArticleFindDataAccess;
use crate::usecase::article_list_get::ArticleListDataAccess;
use crate::usecase::error::DataAccessError;
use crate::usecase::post_create::{self, CreatePostDataAccess};
use crate::usecase::post_delete::DeletePostDataAccess;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct PostUpdateAccess {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl PostUpdateAccess {
    pub fn new(
        title: Option<String>,
        body: Option<String>,
        published: Option<bool>,
    ) -> PostUpdateAccess {
        PostUpdateAccess {
            title,
            body,
            published,
        }
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
struct PostNewAccess<'a> {
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
        PostTable { connection }
    }

    pub fn update(
        &self,
        target_id: i32,
        update_post: PostUpdateAccess,
    ) -> Result<(), diesel::result::Error> {
        let _result = diesel::update(dsl::posts.find(target_id))
            .set(&update_post)
            .get_result::<Post>(self.connection)?;
        Ok(())
    }
}

impl<'a> DataAccess for PostTable<'a> {}

impl<'a> ArticleFindDataAccess for PostTable<'a> {
    fn find(&self, id: i32) -> Result<Option<Post>, DataAccessError> {
        let result = dsl::posts
            .find(id)
            .first::<Post>(self.connection)
            .optional();

        self.parse_data_access_result(result)
    }
}

impl<'a> ArticleListDataAccess for PostTable<'a> {
    fn show(&self, count: i32, page: i32) -> Result<Vec<Post>, DataAccessError> {
        let offset = count * (page - 1);

        let result = dsl::posts
            .filter(dsl::published.eq(true))
            .limit(count as i64)
            .offset(offset as i64)
            .order(dsl::id.desc())
            .load::<Post>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> CreatePostDataAccess for PostTable<'a> {
    fn create(&self, input: post_create::InputData) -> Result<Post, DataAccessError> {
        let new_post = PostNewAccess::new(&input.title, &input.body, input.published);

        let result = diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> DeletePostDataAccess for PostTable<'a> {
    fn delete(&self, target_id: i32) -> Result<(), DataAccessError> {
        let result = diesel::delete(dsl::posts.find(target_id)).execute(self.connection);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::driver::pool::test_util;

    #[test]
    fn scenario() {
        let connection = test_util::connection_init();
        let post_table = PostTable::new(&connection);

        let new_input1 = post_create::InputData {
            title: "unit test title111".to_string(),
            body: "unit test body111".to_string(),
            published: true,
        };
        let created_posts1 = post_table.create(new_input1).unwrap();

        let new_input2 = post_create::InputData {
            title: "unit test title222".to_string(),
            body: "unit test body222".to_string(),
            published: false,
        };
        let created_posts2 = post_table.create(new_input2).unwrap();
        let _published_post = post_table.update(
            created_posts2.id,
            PostUpdateAccess::new(None, None, Some(true)),
        );

        let posts = post_table.show(2, 1).unwrap();

        let result = posts
            .iter()
            .map(|item| item.title.clone())
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);

        let update_post = PostUpdateAccess::new(
            Some("update test title333".to_string()),
            Some("update test body333".to_string()),
            None,
        );
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
