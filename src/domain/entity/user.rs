use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
}
