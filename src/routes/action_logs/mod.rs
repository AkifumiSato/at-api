mod get;
mod post_category;
mod post_record;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post_record::index))
            .route("category/", web::post().to(post_category::index))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::action_record::ActionCategory;
    use actix_web::{test, web, App};
    use crate::database_utils::pool::test_util::setup_connection_pool;

    /// # scenario
    ///
    /// 1. create category
    /// 2. create record
    /// 3. get
    /// 4. TODO impl: patch record
    /// 5. TODO impl: patch category
    /// 6. TODO impl: delete record
    /// 7. TODO impl: delete category
    #[actix_rt::test]
    async fn action_logs_scenario() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .data(web::JsonConfig::default().limit(4096))
                .service(web::scope("").configure(config)),
        )
            .await;

        // todo user id setup

        let category_name_data = "Unit test category";
        let req = test::TestRequest::post()
            .uri("/category/")
            .set_json(&post_category::PostParams {
                user_id: 777,
                name: category_name_data.to_string()
            })
            .to_request();
        let resp: ActionCategory = test::read_response_json(&mut app, req).await;
        assert_eq!(category_name_data, resp.name);
    }
}
