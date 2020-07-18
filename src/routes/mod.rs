mod show_post;
mod create_post;
mod publish_post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/show/", web::post().to(show_post::index))
            .route("/create/", web::post().to(create_post::index))
            .route("/publish/", web::post().to(publish_post::index))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http, web, App, Error};
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::pg::PgConnection;
    use crate::db::{env_database_url, TestTransaction, DbPool};

    #[actix_rt::test]
    async fn test_show() {
        let manager = ConnectionManager::<PgConnection>::new(env_database_url());
        let pool: DbPool = r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .build(manager)
            .expect("Failed to init pool");

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("/").configure(config))
        ).await;

        let req = test::TestRequest::post()
            .uri("/posts/show/")
            .set_json(&show_post::PostJson::new(None, 1))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::post()
            .uri("/posts/show/")
            .set_json(&show_post::PostJson::new(None, 1))
            .to_request();
        let resp: show_post::Response = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp.result.iter().len());
    }
}