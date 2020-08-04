use crate::driver::tags::TagsTable;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub post_id: i32,
    pub tag_id: i32,
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<(), diesel::result::Error> {
    let tags_table = TagsTable::new(&connection);

    tags_table.register_tag_post(input.post_id, input.tag_id)
}
