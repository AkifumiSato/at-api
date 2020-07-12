extern crate my_app;

use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use my_app::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/").configure(routes::config))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
