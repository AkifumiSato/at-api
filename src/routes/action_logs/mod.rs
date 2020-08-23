mod get;
mod post_category;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").route("", web::get().to(get::index)));
    cfg.service(web::scope("/").route("category", web::post().to(post_category::index)));
}
