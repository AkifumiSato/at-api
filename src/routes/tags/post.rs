use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::pool::DbPool;
use crate::db::tags::{TagsTable};
use crate::model::tags::NewTag;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateJsonBody {
    name: String,
    slug: String,
}

impl CreateJsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(name: &str, slug: &str) -> CreateJsonBody {
        CreateJsonBody {
            name: name.to_string(),
            slug: slug.to_string(),
        }
    }

    pub fn to_new_tag(&self) -> NewTag {
        NewTag::new(&self.name, &self.slug)
    }
}

pub async fn create(
    pool: web::Data<DbPool>,
    item: web::Json<CreateJsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.create(item.to_new_tag()) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterJsonBody {
    post_id: i32,
    tag_id: i32,
}

impl RegisterJsonBody {
    /// mod.tsでシナリオテストするために利用.
    #[allow(dead_code)]
    pub fn new(post_id: i32, tag_id: i32) -> RegisterJsonBody {
        RegisterJsonBody {
            post_id,
            tag_id,
        }
    }
}

pub async fn register(
    pool: web::Data<DbPool>,
    item: web::Json<RegisterJsonBody>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");
    let tag_table = TagsTable::new(&connection);

    match tag_table.register_tag_post(item.post_id, item.tag_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}