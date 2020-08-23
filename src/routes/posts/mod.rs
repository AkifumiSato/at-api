mod delete;
mod find;
mod get;
mod patch;
mod post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post::index))
            .route("", web::patch().to(patch::index))
            .route("", web::delete().to(delete::index))
            .route("{id}/", web::get().to(find::index)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_utils::pool::{env_database_url, DbPool, TestTransaction};
    use crate::domain::entity::posts::Post;
    use crate::usecase;
    use actix_web::{test, web, App};
    use diesel::pg::PgConnection;
    use diesel::r2d2::{self, ConnectionManager};

    fn setup_connection_pool() -> DbPool {
        let manager = ConnectionManager::<PgConnection>::new(env_database_url());
        r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .build(manager)
            .expect("Failed to init pool")
    }

    /// # scenario
    ///
    /// 1. create
    /// 2. show
    #[actix_rt::test]
    async fn test_get_posts() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("").configure(config)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post::JsonBody::new(
                "unit test title",
                "unit test body",
                Some(true),
            ))
            .to_request();
        let resp: Post = test::read_response_json(&mut app, req).await;
        let id = resp.id;
        assert_eq!("unit test title", resp.title);
        assert_eq!("unit test body", resp.body);

        let req = test::TestRequest::get().uri("/?count=1").to_request();
        let resp: usecase::articles::get_list::OutputData =
            test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp.result.iter().len());
        assert_eq!(id, resp.result.first().unwrap().id);

        let req = test::TestRequest::get().uri("/").to_request();
        let resp: usecase::articles::get_list::OutputData =
            test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.result.first().unwrap().id);
    }

    /// # scenario
    ///
    /// 1. create
    /// 2. show
    /// 3. update
    /// 4. show
    #[actix_rt::test]
    async fn test_patch() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("").configure(config)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post::JsonBody::new(
                "unit test title",
                "unit test body",
                None,
            ))
            .to_request();
        let resp: Post = test::read_response_json(&mut app, req).await;
        let id = resp.id;
        assert_eq!("unit test title", resp.title);
        assert_eq!("unit test body", resp.body);

        let req = test::TestRequest::get().uri("/?count=1").to_request();
        let resp: usecase::articles::get_list::OutputData =
            test::read_response_json(&mut app, req).await;
        assert_ne!(id, resp.result.first().unwrap().id);

        let req = test::TestRequest::patch()
            .uri("/")
            .set_json(&usecase::articles::post_update::InputData::new(
                id,
                None,
                None,
                Some(true),
            ))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/?count=1").to_request();
        let resp: usecase::articles::get_list::OutputData =
            test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp.result.iter().len());
        assert_eq!(id, resp.result.first().unwrap().id);
    }

    /// # scenario
    ///
    /// 1. create
    /// 2. find
    /// 3. delete
    /// 4. find
    #[actix_rt::test]
    async fn test_delete_find() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("").configure(config)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post::JsonBody::new(
                "unit test title",
                "unit test body",
                Some(true),
            ))
            .to_request();
        let resp: Post = test::read_response_json(&mut app, req).await;
        let id = resp.id;
        assert_eq!("unit test title", resp.title);
        assert_eq!("unit test body", resp.body);

        let req = test::TestRequest::get()
            .uri(&format!("/{}/", id))
            .to_request();
        let resp: usecase::articles::find::OutputData = test::read_response_json(&mut app, req).await;
        let post = resp.result.unwrap();
        assert_eq!("unit test title", post.title);
        assert_eq!("unit test body", post.body);

        let req = test::TestRequest::delete()
            .uri("/")
            .set_json(&usecase::articles::post_delete::InputData::new(id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri(&format!("/{}/", id))
            .to_request();
        let resp: usecase::articles::find::OutputData = test::read_response_json(&mut app, req).await;
        let post = resp.result;
        assert!(post.is_none());
    }
}
