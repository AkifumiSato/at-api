use crate::database_utils::error::DataAccessError;
use crate::domain::entity::posts::Post;
use crate::domain::entity::tags::PostTag;
use chrono::naive::serde::ts_seconds::{deserialize, serialize};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub user_id: i32,
    pub page: i32,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTagOutput {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl PostTag {
    pub fn to_output(&self) -> PostTagOutput {
        PostTagOutput {
            id: self.tag_id,
            name: self.name.clone(),
            slug: self.slug.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostItemOutput {
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
    pub tags: Vec<PostTagOutput>,
}

impl Post {
    pub fn to_output(&self, tags: Vec<PostTagOutput>) -> PostItemOutput {
        PostItemOutput {
            id: self.id,
            title: self.title.clone(),
            body: self.body.clone(),
            published: self.published,
            published_at: self.published_at,
            created_at: self.created_at,
            tags,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<PostItemOutput>,
}

pub trait ArticleListUseCase {
    fn show(&self, input: InputData) -> Result<Vec<Post>, DataAccessError>;
}

pub trait TagFindsUseCase {
    fn find_by_post_ids(&self, post_ids: Vec<i32>) -> Result<Vec<PostTag>, DataAccessError>;
}

pub fn execute<T, U>(
    article_data_access: T,
    tags_data_access: U,
    input: InputData,
) -> Result<OutputData, DataAccessError>
where
    T: ArticleListUseCase,
    U: TagFindsUseCase,
{
    let posts = article_data_access.show(input)?;
    let post_ids = posts.iter().map(|post| post.id).collect::<Vec<i32>>();
    let tags = tags_data_access.find_by_post_ids(post_ids)?;

    let result = posts
        .iter()
        .map(|post| {
            let filtered_tags = tags
                .iter()
                .filter(|tag| tag.post_id == post.id)
                .map(|tag| tag.to_output())
                .collect::<Vec<PostTagOutput>>();

            post.to_output(filtered_tags)
        })
        .collect::<Vec<PostItemOutput>>();

    // let result = post_table.show(input.count, input.page)?;
    Ok(OutputData { result })
}
