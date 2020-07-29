use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use crate::driver::tags::{TagsTable, NewTag};
use crate::domain::entity::tags::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub name: String,
    pub slug: String,
}

impl InputData {
    pub fn to_new_tag(&self) -> NewTag {
        NewTag::new(self.name.clone(), self.slug.clone())
    }
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<Tag, diesel::result::Error> {
    let tags_table = TagsTable::new(&connection);

    tags_table.create(input.to_new_tag())
}