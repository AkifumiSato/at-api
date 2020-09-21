use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct PostTag {
    pub tag_id: i32,
    pub post_id: i32,
    pub name: String,
    pub slug: String,
}
