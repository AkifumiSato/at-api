use serde::{Deserialize, Serialize};
use crate::schema::tags;
use crate::schema::posts_tags;

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    name: &'a str,
    slug: &'a str,
}

impl<'a> NewTag<'a> {
    pub fn new(name: &'a str, slug: &'a str) -> NewTag<'a> {
        NewTag {
            name,
            slug,
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct UpdateTag {
    name: Option<String>,
    slug: Option<String>,
}

impl UpdateTag {
    pub fn new(name: Option<String>, slug: Option<String>) -> UpdateTag {
        UpdateTag {
            name,
            slug,
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
pub struct PostsTag {
    pub post_id: i32,
    pub tag_id: i32,
}