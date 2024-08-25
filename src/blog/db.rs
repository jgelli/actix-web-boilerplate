use std::i32;

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{
    errors::MyError,
    models::{BlogPost, NewBlogPost},
};

pub async fn get_post_by_slug(client: &Client, slug: &str) -> Result<BlogPost, MyError> {
    let stmt = include_str!("sql/get_post_by_slug.sql");
    let stmt = stmt.replace("$table_fields", &BlogPost::sql_table_fields());

    client.prepare(&stmt).await?;

    let row = client.query_one(&stmt, &[&slug]).await?;
    let post = BlogPost::from_row_ref(&row).map_err(MyError::from)?;

    Ok(post)
}

pub async fn add_post(client: &Client, new_blog: BlogPost) -> Result<BlogPost, MyError> {
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

pub async fn get_posts(
    client: &Client,
    last_id: Option<i32>,
    limit: Option<i64>,
    title: Option<String>,
) -> Result<Vec<BlogPost>, MyError> {
    let last_id = last_id.unwrap_or(i32::MAX);
    let limit = limit.unwrap_or(10).min(25);
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&last_id, &limit];

    let mut base_query = include_str!("sql/get_posts.sql").to_string();

    let formatted_title = title.as_ref().map(|t| format!("%{}%", t));
    if formatted_title.is_some() {
        base_query.push_str(" AND title ILIKE $3");
        params.push(&formatted_title);
    }

    base_query.push_str(" ORDER BY id DESC LIMIT $2");

    let stmt = base_query.replace("$table_fields", &BlogPost::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &params)
        .await?
        .iter()
        .map(|row| BlogPost::from_row_ref(row).unwrap())
        .collect::<Vec<BlogPost>>();

    Ok(results)
}

pub async fn update_post(client: &Client, post: &NewBlogPost) -> Result<BlogPost, MyError> {
    let stmt = include_str!("sql/update_post.sql");
    let stmt = stmt.replace("$table_fields", &BlogPost::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let row = client
        .query_one(
            &stmt,
            &[
                &post.id,
                &post.title,
                &post.content,
                &post.feature_image,
                &post.slug,
                &post.author,
            ],
        )
        .await?;
    let post = BlogPost::from_row_ref(&row).map_err(MyError::from)?;

    Ok(post)
}
