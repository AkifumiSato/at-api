use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use crate::driver::tags::TagsTable;
use crate::domain::entity::tags::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<Tag>,
}

pub fn execute(connection: &PgConnection) -> Result<OutputData, diesel::result::Error> {
    let tags_table = TagsTable::new(&connection);

    let result = tags_table.all_tags()?;
    Ok(OutputData {
        result,
    })
}