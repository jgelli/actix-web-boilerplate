use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

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

#[derive(Deserialize, Serialize)]
pub struct NewBlogPost {
    pub title: String,
    pub content: String,
    pub feature_image: Option<String>,
    pub slug: String,
    pub author: String,
}
