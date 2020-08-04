use crate::usecase::error::DataAccessError;

pub trait DataAccess {
    fn parse_data_access_result<T>(&self, result: Result<T, diesel::result::Error>) -> Result<T, DataAccessError> {
        match result {
            Ok(data) => Ok(data),
            Err(_) => Err(DataAccessError::InternalError),
        }
    }
}