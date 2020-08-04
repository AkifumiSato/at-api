use crate::domain::entity::posts::Post;
use crate::driver::posts::{PostNewAccess, PostTable};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<Post, diesel::result::Error> {
    let post_table = PostTable::new(&connection);

    let new_post = PostNewAccess::new(&input.title, &input.body, input.published);
    post_table.create(new_post)
}
