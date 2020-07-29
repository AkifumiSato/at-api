use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::driver::pool::DbPool;
use crate::driver::tags::{TagsTable};
use crate::domain::entity::tags::UpdateTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonBody {
    id: i32,
    name: Option<String>,
    slug: Option<String>,
}

impl JsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new<'a>(id: i32, name: Option<&'a str>, slug: Option<&'a str>) -> JsonBody {
        JsonBody {
            id,
            name: name.map(|v|  v.to_string()),
            slug: slug.map(|v|  v.to_string()),
        }
    }

    pub fn to_update_tag(&self) -> UpdateTag {
        UpdateTag::new(self.name.clone(), self.slug.clone())
    }
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<JsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.update(item.id, item.to_update_tag()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}