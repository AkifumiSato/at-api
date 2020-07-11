extern crate my_app;

use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use my_app::bookmark;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::scope("/bookmark").configure(bookmark::config))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
