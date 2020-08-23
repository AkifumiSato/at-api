pub mod delete;
pub mod post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::post().to(post::index))
            .route("", web::delete().to(delete::index)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_utils::pool::{env_database_url, DbPool, TestTransaction};
    use crate::domain::entity::user::User;
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

        let test_id = 2147483647;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post::JsonBody::new(test_id)) // int max
            .to_request();
        let resp: User = test::read_response_json(&mut app, req).await;
        assert_eq!(test_id, resp.id);

        let req = test::TestRequest::delete()
            .uri("/")
            .set_json(&usecase::users::delete::InputData { id: test_id })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
