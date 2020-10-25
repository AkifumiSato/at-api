mod get;
mod post_record;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post_record::index))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_utils::pool::test_util::setup_connection_pool;
    use actix_web::{test, web, App};
    use crate::driver::users::test_utils::test_user_by_pool;
    use chrono::{Local, Duration};

    #[actix_rt::test]
    async fn attendance_scenario() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("").configure(config)),
        )
            .await;

        let test_user = test_user_by_pool(pool.clone());

        let req = test::TestRequest::get()
            .uri(&format!("/?uid={}", test_user.uid))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);

        let end_time_date = Local::now();
        let start_time_date = end_time_date - Duration::hours(8);
        let break_time = 60 * 60 * 1000;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post_record::PostParams {
                user_id: test_user.id,
                start_time: start_time_date.timestamp(),
                end_time: end_time_date.timestamp(),
                break_time,
            }) // int max
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 201);

        let req = test::TestRequest::get()
            .uri(&format!("/?uid={}", test_user.uid))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
    }
}
