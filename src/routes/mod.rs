mod show_post;
mod create_post;
mod publish_post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/show", web::post().to(show_post::index))
            .route("/create", web::post().to(create_post::index))
            .route("/publish", web::post().to(publish_post::index))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{Error, http, web};
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::pg::PgConnection;
    use crate::db::{env_database_url, TestTransaction, DbPool};

    #[actix_rt::test]
    async fn show_test() -> Result<(), Error> {
        let manager = ConnectionManager::<PgConnection>::new(env_database_url());
        let pool: DbPool = r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .build(manager)
            .expect("Failed to init pool");

        let pool_data = web::Data::new(pool);
        let item_data = web::Json(show_post::PostJson::new(Some(1), 1));
        let resp = show_post::index(pool_data, item_data).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}