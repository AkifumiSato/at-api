use crate::domain::entity::tags::{PostTag, Tag};
use crate::driver::data_access::DataAccess;
use crate::schema::posts_tags;
use crate::schema::tags;
use crate::usecase::article_list_get::TagFindsDataAccess;
use crate::usecase::error::DataAccessError;
use crate::usecase::tag_all_get::TagAllGetDataAccess;
use crate::usecase::tag_create::{self, CreateTagDataAccess};
use crate::usecase::tag_delete::DeleteTagDataAccess;
use crate::usecase::tag_register_to_post::RegisterTagPostDataAccess;
use crate::usecase::tag_update::{self, UpdateTagDataAccess};
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
}

impl NewTag {
    pub fn new(name: String, slug: String) -> NewTag {
        NewTag { name, slug }
    }
}

#[derive(Debug, Queryable, Insertable)]
pub struct PostsTag {
    pub post_id: i32,
    pub tag_id: i32,
}

pub struct TagsTable<'a> {
    connection: &'a PgConnection,
}

impl<'a> TagsTable<'a> {
    pub fn new(connection: &'a PgConnection) -> TagsTable<'a> {
        TagsTable { connection }
    }
}

impl<'a> DataAccess for TagsTable<'a> {}

impl<'a> TagFindsDataAccess for TagsTable<'a> {
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

impl<'a> TagAllGetDataAccess for TagsTable<'a> {
    fn all_tags(&self) -> Result<Vec<Tag>, DataAccessError> {
        let result = tags::dsl::tags
            .distinct_on(tags::id)
            .load::<Tag>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> CreateTagDataAccess for TagsTable<'a> {
    fn create(&self, input: tag_create::InputData) -> Result<Tag, DataAccessError> {
        let result = diesel::insert_into(tags::table)
            .values(NewTag::new(input.name, input.slug))
            .get_result::<Tag>(self.connection);

        self.parse_data_access_result(result)
    }
}

impl<'a> RegisterTagPostDataAccess for TagsTable<'a> {
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

impl<'a> UpdateTagDataAccess for TagsTable<'a> {
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

impl<'a> DeleteTagDataAccess for TagsTable<'a> {
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
    use crate::driver::pool::test_util;
    use crate::driver::posts::PostTable;
    use crate::usecase::post_create;
    use crate::usecase::post_create::CreatePostDataAccess;

    #[test]
    fn tags_scenario() {
        let connection = test_util::connection_init();
        let tags_table = TagsTable::new(&connection);
        let post_table = PostTable::new(&connection);

        let new_input = post_create::InputData {
            title: "unit test title222".to_string(),
            body: "unit test body222".to_string(),
            published: false,
        };
        let created_posts = post_table.create(new_input).unwrap();

        let new_tag = tag_create::InputData {
            name: "test name".to_string(),
            slug: "test slug".to_string(),
        };
        let created_tag = tags_table.create(new_tag).unwrap();
        let _register_result = tags_table.register_tag_post(created_posts.id, created_tag.id);

        let tag = tags_table.find_by_post_ids(vec![created_posts.id]).unwrap();
        let tag = tag.iter().next().unwrap();

        assert_eq!(tag.name, "test name");
        assert_eq!(tag.slug, "test slug");

        let update_tag = tag_update::InputData {
            id: created_tag.id,
            name: Some("update test name111".to_string()),
            slug: Some("update test slug111".to_string()),
        };
        let _result = tags_table.update(update_tag);

        let tag = tags_table.find_by_post_ids(vec![created_posts.id]).unwrap();
        let tag = tag.iter().next().unwrap();

        assert_eq!(tag.name, "update test name111");
        assert_eq!(tag.slug, "update test slug111");

        let all_tags = tags_table.all_tags().unwrap();
        let tag = all_tags
            .iter()
            .filter(|x| x.id == created_tag.id)
            .next()
            .unwrap();

        assert_eq!(tag.slug, "update test slug111");

        let _result = tags_table.delete(created_tag.id);

        let all_tags = tags_table.find_by_post_ids(vec![created_posts.id]);

        assert!(all_tags.is_err());
    }
}
