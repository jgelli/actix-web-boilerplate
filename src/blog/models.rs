use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use validator::Validate;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "testing.blog_posts")]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub feature_image: Option<String>,
    pub slug: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub active: bool,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewBlogPost {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    pub feature_image: Option<String>,

    pub slug: String,

    pub author: String,
}
