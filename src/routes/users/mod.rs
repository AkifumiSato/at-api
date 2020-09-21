pub mod delete;
pub mod get;
pub mod post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post::index))
            .route("", web::delete().to(delete::index)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_utils::pool::test_util::setup_connection_pool;
    use crate::domain::entity::user::User;
    use crate::usecase;
    use actix_web::{test, web, App};

    /// # scenario
    ///
    /// 1. create
    /// 2. get
    /// 3. delete
    /// 3. not found
    #[actix_rt::test]
    async fn user_scenario() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("").configure(config)),
        )
        .await;

        let test_id = "asdfghjkl";

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post::JsonBody::new(test_id.to_string())) // int max
            .to_request();
        let resp: User = test::read_response_json(&mut app, req).await;
        assert_eq!(test_id, resp.uid);

        let req = test::TestRequest::get()
            .uri(&format!("/?uid={}", test_id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);

        let req = test::TestRequest::delete()
            .uri("/")
            .set_json(&usecase::users::delete::InputData {
                uid: test_id.to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri(&format!("/?uid={}", test_id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 204);
    }
}
