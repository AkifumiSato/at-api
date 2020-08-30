use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum DataAccessError {
    InternalError,
    InternalErrorWithMessage(String),
}

impl StdError for DataAccessError {}

impl fmt::Display for DataAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataAccessError::InternalError => write!(f, "data access Error in use case!"),
            DataAccessError::InternalErrorWithMessage(message) => write!(f, "{}", message),
        }
    }
}

pub trait UseCase {
    fn parse_data_access_result<T>(
        &self,
        result: Result<T, diesel::result::Error>,
    ) -> Result<T, DataAccessError> {
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}
