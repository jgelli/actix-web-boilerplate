use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{
    errors::MyError,
    models::{BlogPost, NewBlogPost},
};

pub async fn add_post(client: &Client, new_blog: NewBlogPost) -> Result<BlogPost, MyError> {
    let stmt = include_str!("sql/insert_post.sql");
    let stmt = stmt.replace("$table_fields", &BlogPost::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(
            &stmt,
            &[
                &new_blog.title,
                &new_blog.content,
                &new_blog.feature_image,
                &new_blog.slug,
                &new_blog.author,
            ],
        )
        .await?
        .iter()
        .map(|row| BlogPost::from_row_ref(row).unwrap())
        .collect::<Vec<BlogPost>>()
        .pop()
        .ok_or(MyError::NotFound)
}
