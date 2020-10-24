extern crate my_app;

use actix_ratelimit::errors::ARError::IdentificationError;
use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use my_app::database_utils::pool::env_database_url;
use my_app::routes;
use std::env;
use std::time::Duration;

fn api_key_validate(key: &str) -> bool {
    let env_key = env::var("X_API_KEY").expect("X_API_KEY must be set");
    key == env_key
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_url = env_database_url();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "0.0.0.0:8088";

    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        let store = MemoryStore::new();
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    // 60秒以内のリクエスト件数を制限
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100)
                    // api keyが一致しているかvalidation
                    .with_identifier(|req| {
                        let key = req.headers().get("x-api-key").unwrap();
                        let key = key.to_str().unwrap();
                        if api_key_validate(key) {
                            Ok(key.to_string())
                        } else {
                            println!("[api app log] x-api-key identification failed");
                            Err(IdentificationError)
                        }
                    }),
            )
            .data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/users").configure(routes::users::config))
            .service(web::scope("/attendance_records").configure(routes::action_logs::config))
    })
    .bind(&bind)?
    .run()
    .await
}
