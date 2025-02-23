use serde::Deserialize;
use types::{DateTime, ID, U8I16};

#[derive(sqlx::FromRow, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "RECORD")]
pub struct MyUsers {
    pub id: ID,
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
    pub is_deleted: U8I16,
    pub created_at: DateTime,
}

#[derive(Debug)]
pub struct NewMyUsers {
    pub username: String,
    pub password: String,
    pub roles: Vec<String>,
}
