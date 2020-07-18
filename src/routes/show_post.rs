use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::db::{show_post, Post, DbPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    page: Option<i64>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    result: Vec<Post>,
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<PostJson>
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");

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
    use actix_web::{Error, http, web};
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::pg::PgConnection;
    use crate::db::{env_database_url, TestTransaction};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let manager = ConnectionManager::<PgConnection>::new(env_database_url());
        let pool: DbPool = r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .build(manager)
            .expect("Failed to init pool");

        let pool_data = web::Data::new(pool);
        let item_data = web::Json(PostJson {
            page: Some(1),
            count: 1,
        });
        let resp = index(pool_data, item_data).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}