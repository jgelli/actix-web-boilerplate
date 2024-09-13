use std::i32;

use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{error::Error, models::BlogPost};

pub async fn get_post_by_slug(client: &Client, slug: &str) -> Result<BlogPost, Error> {
    let stmt = include_str!("sql/get_post_by_slug.sql");
    let stmt = stmt.replace("$table_fields", &BlogPost::sql_table_fields());

    client.prepare(&stmt).await?;

    let row = client
        .query_opt(&stmt, &[&slug])
        .await?
        .ok_or(Error::NotFound(format!(
            "Post with slug ({slug}) not found"
        )))?;

    let post = BlogPost::from_row_ref(&row)?;
    Ok(post)
}

pub async fn add_post(client: &Client, new_blog: BlogPost) -> Result<BlogPost, Error> {
    let stmt = include_str!("sql/insert_post.sql");
    let stmt = stmt.replace("$table_fields", &BlogPost::sql_table_fields());
    let stmt = client.prepare(&stmt).await?;
    let row_opt = client
        .query_opt(
            &stmt,
            &[
                &new_blog.title,
                &new_blog.content,
                &new_blog.feature_image,
                &new_blog.slug,
                &new_blog.author,
            ],
        )
        .await?;

    match row_opt {
        Some(row) => {
            let post = BlogPost::from_row_ref(&row)?;
            Ok(post)
        }
        None => Err(Error::UnexpectResult(format!(
            "Failed to insert blog post: {new_blog:#?}"
        ))),
    }
}

pub async fn get_posts(
    client: &Client,
    last_id: i32,
    limit: i64,
    active: bool, //TODO: validar se pode visualizar os posts inativos
    title: &Option<String>,
) -> Result<Vec<BlogPost>, Error> {
    let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
        vec![&last_id, &active, &title, &limit];
    let stmt = include_str!("sql/get_posts.sql").to_string();
    let stmt = client.prepare(&stmt).await.unwrap();

    let posts = client
        .query(&stmt, &params)
        .await?
        .iter()
        .map(BlogPost::from_row_ref)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(posts)
}

pub async fn update_post(client: &Client, post: &BlogPost) -> Result<BlogPost, Error> {
    let stmt = include_str!("sql/update_post.sql");
    let stmt = client.prepare(&stmt).await?;

    let row = client
        .query_one(
            &stmt,
            &[
                &post.id,
                &post.title,
                &post.content,
                &post.feature_image,
                &post.slug,
            ],
        )
        .await?;
    let post = BlogPost::from_row_ref(&row)?;

    Ok(post)
}

pub async fn delete_post(client: &Client, id: &i32) -> Result<BlogPost, Error> {
    let stmt = include_str!("sql/delete_post.sql");
    let stmt = client.prepare(&stmt).await?;

    let row = client.query_one(&stmt, &[&id]).await?;

    let post = BlogPost::from_row_ref(&row)?;

    Ok(post)
}
