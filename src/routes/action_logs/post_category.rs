use crate::database_utils::pool::DbPool;
use crate::driver::action_log::ActionLogDriver;
use crate::usecase::action_records::add_category::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub user_id: i32,
    pub name: String,
}

impl GetParams {
    pub fn to_input_data(&self) -> InputData {
        InputData {
            user_id: self.user_id,
            name: self.name.clone(),
        }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let action_driver = ActionLogDriver::new(&connection);

    match add_category::execute(action_driver, item.to_input_data()) {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
