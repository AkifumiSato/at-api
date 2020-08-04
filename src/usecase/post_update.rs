use crate::driver::posts::{PostTable, PostUpdateAccess};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

impl InputData {
    #[cfg(test)]
    pub fn new<'a>(
        id: i32,
        title: Option<&'a str>,
        body: Option<&'a str>,
        published: Option<bool>,
    ) -> InputData {
        InputData {
            id,
            title: title.map(|v| v.to_string()),
            body: body.map(|v| v.to_string()),
            published,
        }
    }

    pub fn to_update_post(&self) -> PostUpdateAccess {
        PostUpdateAccess::new(
            self.title.clone(),
            self.body.clone(),
            self.published.clone(),
        )
    }
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<(), diesel::result::Error> {
    let post_table = PostTable::new(&connection);

    post_table.update(input.id, input.to_update_post())
}
