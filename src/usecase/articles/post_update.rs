use crate::database_utils::error::DataAccessError;
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
}

pub trait UpdateUseCase {
    fn update(&self, input: InputData) -> Result<(), DataAccessError>;
}

pub fn execute<T>(data_access: T, input: InputData) -> Result<(), DataAccessError>
where
    T: UpdateUseCase,
{
    data_access.update(input)
}
