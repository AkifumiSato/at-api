use crate::users::driver::users::UserTable;
use crate::users::usecase::delete_user::{self, InputData};
use crate::database_utils::pool::DbPool;
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, item: web::Json<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let user_table = UserTable::new(&connection);
    let input = item.into_inner();
    let id = input.id;

    match delete_user::execute(user_table, input) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete post [{}]", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
