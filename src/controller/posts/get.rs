use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::driver::pool::DbPool;
use crate::usecase::article_list_get::{self, InputData};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub page: Option<i32>,
    pub count: Option<i32>,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        let page = self.page.unwrap_or_else(|| 1);
        let count = self.count.unwrap_or_else(|| 10);

        InputData {
            page,
            count,
        }
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Query<GetParams>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");

    match article_list_get::execute(&connection, item.into_inner().to_input_data()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}