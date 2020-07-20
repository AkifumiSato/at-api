mod show;
mod create;
mod publish;
mod update;
mod delete;
mod find;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/show/", web::post().to(show::index))
            .route("/find/", web::post().to(find::index))
            .route("/create/", web::post().to(create::index))
            .route("/publish/", web::post().to(publish::index))
            .route("/update/", web::post().to(update::index))
            .route("/delete/", web::post().to(delete::index))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::pg::PgConnection;
    use crate::db::pool::{env_database_url, TestTransaction, DbPool};
    use crate::db::posts;

    fn setup_connection_pool() -> DbPool  {
        let manager = ConnectionManager::<PgConnection>::new(env_database_url());
        r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .build(manager)
            .expect("Failed to init pool")
    }

    /// # scenario
    ///
    /// 1. create
    /// 2. publish
    /// 3. show
    /// 4. update
    /// 5. delete
    #[actix_rt::test]
    async fn test_scenario() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("/").configure(config))
        ).await;

        let req = test::TestRequest::post()
            .uri("/posts/create/")
            .set_json(&create::PostJson::new("unit test title", "unit test body", None))
            .to_request();
        let resp: posts::Post = test::read_response_json(&mut app, req).await;
        let id = resp.id;
        assert_eq!("unit test title", resp.title);
        assert_eq!("unit test body", resp.body);

        let req = test::TestRequest::post()
            .uri("/posts/publish/")
            .set_json(&publish::PostJson::new(id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::post()
            .uri("/posts/show/")
            .set_json(&show::PostJson::new(None, 1))
            .to_request();
        let resp: show::Response = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp.result.iter().len());
        assert_eq!(id, resp.result.first().unwrap().id);

        let req = test::TestRequest::post()
            .uri("/posts/update/")
            .set_json(&update::PostJson::new(id, Some("update test title"), Some("update test body"), Some(true)))
            .to_request();
        let resp: posts::Post = test::read_response_json(&mut app, req).await;
        assert_eq!("update test title", resp.title);
        assert_eq!("update test body", resp.body);

        let req = test::TestRequest::post()
            .uri("/posts/delete/")
            .set_json(&publish::PostJson::new(id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    /// # scenario2
    ///
    /// 1. create
    /// 2. find
    #[actix_rt::test]
    async fn test_scenario2() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("/").configure(config))
        ).await;

        let req = test::TestRequest::post()
            .uri("/posts/create/")
            .set_json(&create::PostJson::new("unit test title", "unit test body", Some(true)))
            .to_request();
        let resp: posts::Post = test::read_response_json(&mut app, req).await;
        let id = resp.id;
        assert_eq!("unit test title", resp.title);
        assert_eq!("unit test body", resp.body);

        let req = test::TestRequest::post()
            .uri("/posts/find/")
            .set_json(&find::PostJson::new(id))
            .to_request();
        let resp: find::Response = test::read_response_json(&mut app, req).await;
        let post = resp.result.unwrap();
        assert_eq!("unit test title", post.title);
        assert_eq!("unit test body", post.body);
    }
}