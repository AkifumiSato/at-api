use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::db::{establish_connection, show_post, Post};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    page: Option<i64>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    result: Vec<Post>,
}

pub async fn index(item: web::Json<PostJson>) -> HttpResponse {
    let connection = establish_connection();

    let page = match item.page {
        Some(x) => x,
        None => 1,
    };

    match show_post(&connection, item.count, page) {
        Ok(posts) => HttpResponse::Ok().json(Response {
            result: posts
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, Error, web, App};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().route("/", web::post().to(index)),
        )
            .await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&MyObj {
                name: "my-name".to_owned(),
                number: 43,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}