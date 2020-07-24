use chrono::{NaiveDateTime};
use chrono::naive::serde::ts_seconds::{serialize, deserialize};
use serde::{Deserialize, Serialize};
use crate::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    title: &'a str,
    body: &'a str,
    published: bool,
}

impl<'a> NewPost<'a> {
    pub fn new(title: &'a str, body: &'a str, published: bool) -> NewPost<'a> {
        NewPost {
            title,
            body,
            published,
        }
    }
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl UpdatePost {
    pub fn new(title: Option<String>, body: Option<String>, published: Option<bool>) -> UpdatePost {
        UpdatePost {
            title,
            body,
            published,
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize")]
    #[serde(deserialize_with = "deserialize")]
    pub published_at: NaiveDateTime,
}