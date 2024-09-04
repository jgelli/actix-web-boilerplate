use chrono::{DateTime, Utc};
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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewBlogPost {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    pub feature_image: Option<String>,

    pub author: String,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateBlogPost {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    pub feature_image: Option<String>,

    #[serde(default = "default_update_slug")]
    pub update_slug: bool,
}

fn default_update_slug() -> bool {
    false
}

impl BlogPost {
    pub fn new(new_post: NewBlogPost, slug: String) -> Self {
        BlogPost {
            id: 0,
            title: new_post.title,
            content: new_post.content,
            feature_image: new_post.feature_image,
            slug,
            author: new_post.author,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            active: false,
        }
    }

    pub fn update_from(&mut self, updated_post: UpdateBlogPost, new_slug: Option<String>) {
        self.title = updated_post.title;
        self.content = updated_post.content;
        self.feature_image = updated_post.feature_image;

        if let Some(slug) = new_slug {
            self.slug = slug;
        }
    }
}
