pub mod post;
pub mod delete;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::post().to(post::index))
            .route("", web::delete().to(delete::index)),
    );
}