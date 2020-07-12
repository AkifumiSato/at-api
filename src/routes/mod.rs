pub mod show_post;
pub mod create_post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("/show", web::post().to(show_post::index))
            .route("/create", web::post().to(create_post::index))
    );
}