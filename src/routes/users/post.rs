use crate::database_utils::pool::DbPool;
use crate::driver::users::UserDriver;
use crate::usecase::users::add::{self as add_user, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    id: i32,
}

impl JsonBody {
    #[cfg(test)]
    pub fn new(id: i32) -> JsonBody {
        JsonBody { id }
    }

    pub fn to_input_data(&self) -> InputData {
        InputData { id: self.id }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Json<JsonBody>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let user_driver = UserDriver::new(&connection);

    match add_user::execute(user_driver, item.to_input_data()) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_e) => HttpResponse::InternalServerError()
            .body("error occurred by duplicate id or unexpectedly"),
    }
}
