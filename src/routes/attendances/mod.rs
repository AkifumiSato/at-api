mod get;
mod patch;
mod post_record;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::route))
            .route("", web::post().to(post_record::route))
            .route("", web::patch().to(patch::route)),
    );
}

// noinspection DuplicatedCode
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_utils::pool::test_util::setup_connection_pool;
    use crate::domain::entity::attendance_record::AttendanceRecord;
    use crate::driver::users::test_utils::test_user_by_pool;
    use actix_web::{test, web, App};
    use chrono::{Duration, Local, NaiveDateTime};

    #[actix_rt::test]
    async fn attendance_status_test() {
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
                uid: test_user.uid.clone(),
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

    #[actix_rt::test]
    async fn attendance_scenario_test() {
        let pool = setup_connection_pool();

        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .service(web::scope("").configure(config)),
        )
        .await;

        let test_user = test_user_by_pool(pool.clone());

        let end_time_date = Local::now();
        let start_time_date = end_time_date - Duration::hours(8);
        let break_time = 60 * 60 * 1000;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&post_record::PostParams {
                uid: test_user.uid.clone(),
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
        let resp_records: Vec<AttendanceRecord> = test::read_response_json(&mut app, req).await;
        assert!(!resp_records.is_empty());
        let resp_record = resp_records.first().unwrap();

        // update
        let end_time2_date = Local::now();
        let start_time2_date = end_time_date - Duration::hours(8);
        let break_time2 = 60 * 60 * 1000;

        let req = test::TestRequest::patch()
            .uri("/")
            .set_json(&patch::JsonBody {
                uid: test_user.uid.clone(),
                id: resp_record.id,
                start_time: Some(start_time2_date.timestamp()),
                end_time: Some(end_time2_date.timestamp()),
                break_time: Some(break_time2),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status().as_u16(), 204);

        let req = test::TestRequest::get()
            .uri(&format!("/?uid={}", test_user.uid))
            .to_request();
        let resp_records: Vec<AttendanceRecord> = test::read_response_json(&mut app, req).await;
        assert!(!resp_records.is_empty());
        let resp_record = resp_records.first().unwrap();
        assert_eq!(
            resp_record.start_time,
            NaiveDateTime::from_timestamp(start_time2_date.timestamp(), 0)
        );
        assert_eq!(
            resp_record.end_time,
            NaiveDateTime::from_timestamp(end_time2_date.timestamp(), 0)
        );
        assert_eq!(resp_record.break_time, break_time2);
    }
}
