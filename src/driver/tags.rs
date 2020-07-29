use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::tags;
use crate::schema::posts_tags;
use crate::domain::entity::tags::Tag;

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct TagUpdateAccess {
    name: Option<String>,
    slug: Option<String>,
}

impl TagUpdateAccess {
    pub fn new(name: Option<String>, slug: Option<String>) -> TagUpdateAccess {
        TagUpdateAccess {
            name,
            slug,
        }
    }
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    name: String,
    slug: String,
}

impl NewTag {
    pub fn new(name: String, slug: String) -> NewTag {
        NewTag {
            name,
            slug,
        }
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
        TagsTable {
            connection,
        }
    }

    pub fn create(&self, tags: NewTag) -> Result<Tag, diesel::result::Error> {
        diesel::insert_into(tags::table)
            .values(tags)
            .get_result::<Tag>(self.connection)
    }

    pub fn register_tag_post(&self, post_id: i32, tag_id: i32) -> Result<(), diesel::result::Error> {
        diesel::insert_into(posts_tags::table)
            .values(PostsTag {
                post_id,
                tag_id,
            })
            .execute(self.connection)?;
        Ok(())
    }

    pub fn find_by_post(&self, post_ids: Vec<i32>) -> Result<Vec<Tag>, diesel::result::Error> {
        let tag_ids: Vec<i32> = posts_tags::dsl::posts_tags
            .filter(posts_tags::dsl::post_id.eq_any(post_ids))
            .select(posts_tags::tag_id)
            .load::<i32>(self.connection)?;

        tags::dsl::tags
            .filter(tags::id.eq_any(tag_ids))
            .load::<Tag>(self.connection)
    }

    pub fn all_tags(&self) -> Result<Vec<Tag>, diesel::result::Error> {
        tags::dsl::tags
            .distinct_on(tags::id)
            .load::<Tag>(self.connection)
    }

    pub fn update(&self, target_id: i32, update_tag: TagUpdateAccess) -> Result<(), diesel::result::Error> {
        let _result = diesel::update(tags::dsl::tags.find(target_id))
            .set(&update_tag)
            .get_result::<Tag>(self.connection)?;
        Ok(())
    }

    pub fn delete(&self, target_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(tags::dsl::tags.find(target_id))
            .execute(self.connection)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::driver::pool::{test_util};
    use crate::driver::posts::{PostTable, PostNewAccess};

    #[test]
    fn tags_scenario() {
        let connection = test_util::connection_init();
        let tags_table = TagsTable::new(&connection);
        let post_table = PostTable::new(&connection);

        let new_post = PostNewAccess::new("tag test title", "tag test body", true);
        let created_posts = post_table.create(new_post).unwrap();

        let new_tag = NewTag::new("test name".to_string(), "test slug".to_string());
        let created_tag = tags_table.create(new_tag).unwrap();
        let _register_result = tags_table.register_tag_post(created_posts.id, created_tag.id);

        let tag = tags_table
            .find_by_post(vec![created_posts.id])
            .unwrap();
        let tag = tag
            .iter()
            .next()
            .unwrap();

        assert_eq!(tag.name, "test name");
        assert_eq!(tag.slug, "test slug");

        let update_tag = TagUpdateAccess::new(Some("update test name111".to_string()), Some("update test slug111".to_string()));
        let _result = tags_table.update(created_tag.id, update_tag);

        let tag = tags_table
            .find_by_post(vec![created_posts.id])
            .unwrap();
        let tag = tag
            .iter()
            .next()
            .unwrap();

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

        let all_tags = tags_table
            .find_by_post(vec![created_posts.id]);

        assert!(all_tags.is_err());
    }
}