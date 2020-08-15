use crate::articles::driver::posts::PostTable;
use crate::articles::driver::tags::TagsTable;
use crate::articles::usecase::article_list_get::{self, InputData};
use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub page: Option<i32>,
    pub count: Option<i32>,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        let page = self.page.unwrap_or_else(|| 1);
        let count = self.count.unwrap_or_else(|| 10);

        InputData { page, count }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_table = PostTable::new(&connection);
    let tags_table = TagsTable::new(&connection);

    match article_list_get::execute(post_table, tags_table, item.into_inner().to_input_data()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}