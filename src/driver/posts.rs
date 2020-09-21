use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::posts::Post;
use crate::schema::posts;
use crate::schema::posts::dsl;
use crate::usecase::articles::find::{self, ArticleFindUseCase};
use crate::usecase::articles::get_list::{self, ArticleListUseCase};
use crate::usecase::articles::post_create::{self, CreatePostUseCase};
use crate::usecase::articles::post_delete::DeletePostUseCase;
use crate::usecase::articles::post_update::{self, UpdateUseCase};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::driver::common::get_registered_user;

#[derive(Insertable)]
#[table_name = "posts"]
struct PostNewAccess<'a> {
    user_id: i32,
    title: &'a str,
    body: &'a str,
    published: bool,
}

impl<'a> PostNewAccess<'a> {
    pub fn from_input(id: i32, input: &'a post_create::InputData) -> PostNewAccess<'a> {
        PostNewAccess {
            user_id: id,
            title: &input.title,
            body: &input.body,
            published: input.published,
        }
    }
}

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

pub struct PostDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> PostDriver<'a> {
        PostDriver { connection }
    }
}

impl<'a> UseCase for PostDriver<'a> {}

impl<'a> ArticleFindUseCase for PostDriver<'a> {
    fn find(&self, input: find::InputData) -> Result<Option<Post>, DataAccessError> {
        // user registered check
        let _user = get_registered_user(self.connection, input.uid.clone())?;

        let result = dsl::posts
            .find(input.id)
            .first::<Post>(self.connection)
            .optional();

        self.parse_data_access_result(result)
    }
}

impl<'a> ArticleListUseCase for PostDriver<'a> {
    fn show(&self, input: get_list::InputData) -> Result<Vec<Post>, DataAccessError> {
        let offset = input.count * (input.page - 1);

        let result = dsl::posts
            .filter(dsl::user_id.eq(input.user_id))
            .filter(dsl::published.eq(true))
            .limit(input.count as i64)
            .offset(offset as i64)
            .order(dsl::id.desc())
            .load::<Post>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> CreatePostUseCase for PostDriver<'a> {
    fn create(&self, input: post_create::InputData) -> Result<Post, DataAccessError> {
        let user = get_registered_user(self.connection, input.user_id.clone())?;

        let new_post = PostNewAccess::from_input(user.id, &input);

        let result = diesel::insert_into(posts::table)
            .values(new_post)
            .get_result::<Post>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> DeletePostUseCase for PostDriver<'a> {
    fn delete(&self, target_id: i32) -> Result<(), DataAccessError> {
        let result = diesel::delete(dsl::posts.find(target_id)).execute(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

impl<'a> UpdateUseCase for PostDriver<'a> {
    fn update(&self, input: post_update::InputData) -> Result<(), DataAccessError> {
        let result = diesel::update(dsl::posts.find(input.id))
            .set(PostUpdateAccess::new(
                input.title,
                input.body,
                input.published,
            ))
            .get_result::<Post>(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util;
    use crate::driver::users::test_utils::test_user_by_connection;

    #[test]
    fn scenario() {
        let connection = test_util::connection_init();
        let post_driver = PostDriver::new(&connection);
        let user = test_user_by_connection(&connection);

        let new_input1 = post_create::InputData {
            user_id: user.uid.clone(),
            title: "unit test title111".to_string(),
            body: "unit test body111".to_string(),
            published: true,
        };
        let created_posts1 = post_driver.create(new_input1).unwrap();

        let new_input2 = post_create::InputData {
            user_id: user.uid.clone(),
            title: "unit test title222".to_string(),
            body: "unit test body222".to_string(),
            published: false,
        };
        let created_posts2 = post_driver.create(new_input2).unwrap();
        let _published_post = post_driver.update(post_update::InputData::new(
            created_posts2.id,
            None,
            None,
            Some(true),
        ));

        let posts = post_driver
            .show(get_list::InputData {
                user_id: user.id,
                page: 1,
                count: 2,
            })
            .unwrap();

        let result = posts
            .iter()
            .map(|item| item.title.clone())
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);

        let update_post = post_update::InputData::new(
            created_posts2.id,
            Some("update test title333"),
            Some("update test body333"),
            None,
        );
        let _result = post_driver.update(update_post);
        let posts = post_driver
            .show(get_list::InputData {
                user_id: user.id,
                page: 1,
                count: 1,
            })
            .unwrap();

        assert_eq!(posts.first().unwrap().title, "update test title333");
        assert_eq!(posts.first().unwrap().body, "update test body333");

        let _result = post_driver.delete(created_posts2.id);
        let posts = post_driver
            .show(get_list::InputData {
                user_id: user.id,
                page: 1,
                count: 1,
            })
            .unwrap();
        assert_ne!(posts.first().unwrap().title, "update test title333");

        let result = post_driver.find(find::InputData::new(created_posts1.id, user.uid.clone())).unwrap().unwrap();
        assert_eq!(result.title, "unit test title111");

        let result = post_driver.find(find::InputData::new(created_posts2.id, user.uid.clone())).unwrap();
        assert!(result.is_none());
    }
}
