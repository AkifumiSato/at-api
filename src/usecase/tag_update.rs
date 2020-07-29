use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use crate::driver::tags::{TagsTable, TagUpdateAccess};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
}

impl InputData {
    pub fn to_update_tag(&self) -> TagUpdateAccess {
        TagUpdateAccess::new(self.name.clone(), self.slug.clone())
    }
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<(), diesel::result::Error> {
    let tags_table = TagsTable::new(&connection);

    tags_table.update(input.id, input.to_update_tag())
}