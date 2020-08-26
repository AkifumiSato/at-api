use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub page: Option<i32>,
    pub count: Option<i32>,
}

pub async fn index(_pool: web::Data<DbPool>, _item: web::Query<GetParams>) -> HttpResponse {
    HttpResponse::Ok().body("Hello, Action log.")
    // todo: get_records::executeする
}
