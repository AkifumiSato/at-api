use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use crate::driver::tags::{TagsTable};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<(), diesel::result::Error> {
    let tags_table = TagsTable::new(&connection);

    tags_table.delete(input.id)
}