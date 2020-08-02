use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum DataAccessError {
    InternalError,
}

impl StdError for DataAccessError {}

impl fmt::Display for DataAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "data access Error in use case!")
    }
}