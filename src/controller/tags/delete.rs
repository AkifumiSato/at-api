use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::driver::pool::DbPool;
use crate::driver::tags::TagsTable;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    id: i32,
}

impl JsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(id: i32) -> JsonBody {
        JsonBody {
            id,
        }
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<JsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.delete(item.id) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete tag. Id is [{}]", item.id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}