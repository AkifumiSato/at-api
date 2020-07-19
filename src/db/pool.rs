use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, CustomizeConnection};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn env_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
}

/// テスト時にCommitしないtransactionを提供するtrait.
///
/// # example
///
/// ```
/// use actix_web::{Error, http, web};
/// use diesel::r2d2::{self, ConnectionManager};
/// use diesel::pg::PgConnection;
/// use my_app::db::pool::{env_database_url, TestTransaction, DbPool};
///
/// let manager = ConnectionManager::<PgConnection>::new(env_database_url());
/// let pool: DbPool = r2d2::Pool::builder()
///     .connection_customizer(Box::new(TestTransaction))
///     .build(manager)
///     .expect("Failed to init pool");
/// ```
#[derive(Debug)]
pub struct TestTransaction;

impl CustomizeConnection<PgConnection, r2d2::Error> for TestTransaction {
    fn on_acquire(
        &self,
        conn: &mut PgConnection,
    ) -> ::std::result::Result<(), r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}