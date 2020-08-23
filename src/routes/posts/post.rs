use crate::database_utils::pool::DbPool;
use crate::driver::posts::PostDriver;
use crate::usecase::articles::post_create::{self, InputData};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    title: String,
    body: String,
    published: Option<bool>,
}

impl JsonBody {
    #[cfg(test)]
    pub fn new(title: &str, body: &str, published: Option<bool>) -> JsonBody {
        JsonBody {
            title: title.to_string(),
            body: body.to_string(),
            published,
        }
    }

    pub fn to_input_data(&self) -> InputData {
        let published = match self.published {
            Some(value) => value,
            None => false,
        };

        InputData {
            title: self.title.to_string(),
            body: self.body.to_string(),
            published,
        }
    }
}

pub async fn index(pool: web::Data<DbPool>, item: web::Json<JsonBody>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_table = PostDriver::new(&connection);

    match post_create::execute(post_table, item.to_input_data()) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
