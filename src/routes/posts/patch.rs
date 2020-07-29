use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::posts::{PostTable};
use crate::db::pool::DbPool;
use crate::domain::entity::posts::UpdatePost;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    id: i32,
    title: Option<String>,
    body: Option<String>,
    published: Option<bool>,
}

impl JsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new<'a>(id: i32, title: Option<&'a str>, body: Option<&'a str>, published: Option<bool>) -> JsonBody {
        JsonBody {
            id,
            title: title.map(|v|  v.to_string()),
            body: body.map(|v|  v.to_string()),
            published,
        }
    }

    pub fn to_update_post(&self) -> UpdatePost {
        UpdatePost::new(self.title.clone(), self.body.clone(), self.published.clone())
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<JsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let post_table = PostTable::new(&connection);

    match post_table.update(item.id, item.to_update_post()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}