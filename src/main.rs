extern crate my_app;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use my_app::database_utils::pool::env_database_url;
use my_app::routes;

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
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/users").configure(routes::users::config))
            .service(web::scope("/posts").configure(routes::posts::config))
            .service(web::scope("/tags").configure(routes::tags::config))
            .service(web::scope("/action_logs").configure(routes::action_logs::config))
    })
    .bind(&bind)?
    .run()
    .await
}
