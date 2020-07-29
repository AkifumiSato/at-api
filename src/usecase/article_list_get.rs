use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use crate::driver::posts::PostTable;
use crate::domain::entity::posts::{Post,};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub page: i32,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub result: Vec<Post>,
}

pub fn execute(connection: &PgConnection, input: InputData) -> Result<OutputData, diesel::result::Error> {
    let post_table = PostTable::new(&connection);

    let result = post_table.show(input.count, input.page)?;
    Ok(OutputData {
        result,
    })
}