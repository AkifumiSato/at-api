extern crate my_app;

use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use my_app::routes;
use my_app::db::pool::env_database_url;

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
            .service(web::scope("/posts").configure(routes::posts::config))
            .service(web::scope("/tags").configure(routes::tags::config))
    })
        .bind(&bind)?
        .run()
        .await
}
