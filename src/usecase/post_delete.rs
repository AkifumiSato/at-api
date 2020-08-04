use crate::driver::posts::PostTable;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub id: i32,
}

impl InputData {
    #[cfg(test)]
    pub fn new<'a>(id: i32) -> InputData {
        InputData { id }
    }
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<(), diesel::result::Error> {
    let post_table = PostTable::new(&connection);

    post_table.delete(input.id)
}
