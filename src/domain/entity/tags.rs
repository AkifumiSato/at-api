use serde::{Deserialize, Serialize};
use crate::schema::tags;
use crate::schema::posts_tags;

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