use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, CustomizeConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn env_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

/// テスト時にCommitしないtransactionを提供するtrait.
///
/// # example
///
/// ```
/// use actix_web::{Error, http, web};
/// use diesel::r2d2::{self, ConnectionManager};
/// use diesel::pg::PgConnection;
/// use my_app::driver::pool::{env_database_url, TestTransaction, DbPool};
///
/// let manager = ConnectionManager::<PgConnection>::new(env_database_url());
/// let pool_builder = r2d2::Pool::<ConnectionManager<PgConnection>>::builder()
///     .connection_customizer(Box::new(TestTransaction))
///     .max_size(1);
///
/// // let pool = pool_builder.build(manager);
/// ```
#[derive(Debug)]
pub struct TestTransaction;

impl CustomizeConnection<PgConnection, r2d2::Error> for TestTransaction {
    fn on_acquire(&self, conn: &mut PgConnection) -> ::std::result::Result<(), r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

#[cfg(test)]
pub mod test_util {
    use super::*;
    use crate::driver::pool::env_database_url;

    pub fn connection_init() -> PgConnection {
        let database_url = env_database_url();
        let db = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        db.begin_test_transaction().unwrap();
        db
    }
}
