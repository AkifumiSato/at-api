use crate::database_utils::pool::DbPool;
use crate::driver::post_tags::PostTagDriver;
use crate::driver::posts::PostDriver;
use crate::usecase::articles::get_list::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    user_id: i32,
    page: Option<i32>,
    count: Option<i32>,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        let page = self.page.unwrap_or_else(|| 1);
        let count = self.count.unwrap_or_else(|| 10);

        InputData {
            user_id: self.user_id,
            page,
            count
        }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_driver = PostDriver::new(&connection);
    let tags_driver = PostTagDriver::new(&connection);

    match get_list::execute(post_driver, tags_driver, item.into_inner().to_input_data()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
