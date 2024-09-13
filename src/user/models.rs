use serde::{Deserialize, Serialize};
use tokio_postgres::row::Row;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    pub fn from_row_ref(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
        })
    }
}
