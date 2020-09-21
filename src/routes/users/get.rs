use crate::database_utils::pool::DbPool;
use crate::driver::users::UserDriver;
use crate::usecase::users::check;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetParams {
    pub uid: String,
}

pub async fn index(pool: web::Data<DbPool>, item: web::Query<GetParams>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let users_driver = UserDriver::new(&connection);

    match check::execute(users_driver, item.uid.clone()) {
        Ok(result) => match result {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NoContent().finish(),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
