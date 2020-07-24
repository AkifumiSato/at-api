mod get;
mod post;
mod patch;
mod delete;
mod find;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(get::index))
            .route("", web::post().to(post::create))
            .route("register/", web::post().to(post::register))
            .route("", web::patch().to(patch::index))
            .route("", web::delete().to(delete::index))
            .route("{id}/", web::get().to(find::index))
    );
}