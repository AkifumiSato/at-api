extern crate my_app;

use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use my_app::users::routes as users_route;
use my_app::articles::routes as articles_route;
use my_app::action_logs::routes as action_logs_route;
use my_app::database_utils::pool::env_database_url;

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
            .service(web::scope("/users").configure(users_route::config))
            .service(web::scope("/posts").configure(articles_route::posts::config))
            .service(web::scope("/tags").configure(articles_route::tags::config))
            .service(web::scope("/action_logs").configure(action_logs_route::config))
    })
    .bind(&bind)?
    .run()
    .await
}
