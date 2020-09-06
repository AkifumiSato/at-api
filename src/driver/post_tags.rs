use crate::database_utils::error::{DataAccessError, UseCase};
use crate::domain::entity::tags::{PostTag, Tag};
use crate::schema::posts_tags;
use crate::schema::tags;
use crate::usecase::articles::get_list::TagFindsUseCase;
use crate::usecase::articles::tag_all_get::TagAllGetUseCase;
use crate::usecase::articles::tag_create::{self, CreateTagUseCase};
use crate::usecase::articles::tag_delete::DeleteTagUseCase;
use crate::usecase::articles::tag_register_to_post::RegisterTagPostUseCase;
use crate::usecase::articles::tag_update::{self, UpdateTagUseCase};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(AsChangeset)]
#[table_name = "tags"]
struct UpdateTag {
    name: Option<String>,
    slug: Option<String>,
}

impl UpdateTag {
    pub fn new(name: Option<String>, slug: Option<String>) -> UpdateTag {
        UpdateTag { name, slug }
    }
}

#[derive(Insertable)]
#[table_name = "tags"]
struct NewTag {
    name: String,
    slug: String,
    user_id: i32,
}

impl NewTag {
    pub fn from_input(input: tag_create::InputData) -> NewTag {
        NewTag {
            user_id: input.user_id,
            name: input.name,
            slug: input.slug,
        }
    }
}

#[derive(Debug, Queryable, Insertable)]
pub struct PostsTag {
    pub post_id: i32,
    pub tag_id: i32,
}

pub struct PostTagDriver<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostTagDriver<'a> {
    pub fn new(connection: &'a PgConnection) -> PostTagDriver<'a> {
        PostTagDriver { connection }
    }
}

impl<'a> UseCase for PostTagDriver<'a> {}

impl<'a> TagFindsUseCase for PostTagDriver<'a> {
    fn find_by_post_ids(&self, post_ids: Vec<i32>) -> Result<Vec<PostTag>, DataAccessError> {
        let result = posts_tags::dsl::posts_tags
            .filter(posts_tags::dsl::post_id.eq_any(post_ids))
            .inner_join(tags::dsl::tags.on(tags::dsl::id.eq(posts_tags::dsl::tag_id)))
            .select((
                posts_tags::tag_id,
                posts_tags::post_id,
                tags::dsl::name,
                tags::dsl::slug,
            ))
            .load::<PostTag>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> TagAllGetUseCase for PostTagDriver<'a> {
    fn all_tags(&self) -> Result<Vec<Tag>, DataAccessError> {
        let result = tags::dsl::tags
            .distinct_on(tags::id)
            .load::<Tag>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> CreateTagUseCase for PostTagDriver<'a> {
    fn create(&self, input: tag_create::InputData) -> Result<Tag, DataAccessError> {
        let result = diesel::insert_into(tags::table)
            .values(NewTag::from_input(input))
            .get_result::<Tag>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> RegisterTagPostUseCase for PostTagDriver<'a> {
    fn register_tag_post(&self, post_id: i32, tag_id: i32) -> Result<(), DataAccessError> {
        let result = diesel::insert_into(posts_tags::table)
            .values(PostsTag { post_id, tag_id })
            .execute(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

impl<'a> UpdateTagUseCase for PostTagDriver<'a> {
    fn update(&self, input: tag_update::InputData) -> Result<(), DataAccessError> {
        let result = diesel::update(tags::dsl::tags.find(input.id))
            .set(UpdateTag::new(input.name, input.slug))
            .get_result::<Tag>(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

impl<'a> DeleteTagUseCase for PostTagDriver<'a> {
    fn delete(&self, target_id: i32) -> Result<(), DataAccessError> {
        let result = diesel::delete(tags::dsl::tags.find(target_id)).execute(self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database_utils::pool::test_util::setup_connection_pool;
    use crate::driver::posts::PostDriver;
    use crate::driver::users::test_utils::test_user_by_connection;
    use crate::usecase::articles::post_create;
    use crate::usecase::articles::post_create::CreatePostUseCase;

    #[test]
    fn tags_scenario() {
        let pool = setup_connection_pool();
        let connection = pool.get().unwrap();
        let user = test_user_by_connection(&connection);
        let tags_driver = PostTagDriver::new(&connection);
        let post_driver = PostDriver::new(&connection);

        let new_input = post_create::InputData {
            user_id: user.id,
            title: "unit test title222".to_string(),
            body: "unit test body222".to_string(),
            published: false,
        };
        let created_posts = post_driver.create(new_input).unwrap();

        let new_tag = tag_create::InputData {
            user_id: user.id,
            name: "test name".to_string(),
            slug: "test slug".to_string(),
        };
        let created_tag = tags_driver.create(new_tag).unwrap();
        let _register_result = tags_driver.register_tag_post(created_posts.id, created_tag.id);

        let tag = tags_driver
            .find_by_post_ids(vec![created_posts.id])
            .unwrap();
        let tag = tag.iter().next().unwrap();

        assert_eq!(tag.name, "test name");
        assert_eq!(tag.slug, "test slug");

        let update_tag = tag_update::InputData {
            id: created_tag.id,
            name: Some("update test name111".to_string()),
            slug: Some("update test slug111".to_string()),
        };
        let _result = tags_driver.update(update_tag);

        let tag = tags_driver
            .find_by_post_ids(vec![created_posts.id])
            .unwrap();
        let tag = tag.iter().next().unwrap();

        assert_eq!(tag.name, "update test name111");
        assert_eq!(tag.slug, "update test slug111");

        let all_tags = tags_driver.all_tags().unwrap();
        let tag = all_tags
            .iter()
            .filter(|x| x.id == created_tag.id)
            .next()
            .unwrap();

        assert_eq!(tag.slug, "update test slug111");

        let _result = tags_driver.delete(created_tag.id);

        let all_tags = tags_driver.find_by_post_ids(vec![created_posts.id]);

        assert!(all_tags.is_err());
    }
}
