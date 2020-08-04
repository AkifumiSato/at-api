mod delete;
mod get;
mod patch;
mod post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post::create))
            .route("register/", web::post().to(post::register))
            .route("", web::patch().to(patch::index))
            .route("", web::delete().to(delete::index)),
    );
}
