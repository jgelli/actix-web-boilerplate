use std::collections::HashMap;

use actix_web::{get, post, put, web, Error, HttpResponse, Responder, Scope};
use deadpool_postgres::Pool;
use validator::Validate;

use super::{
    db,
    errors::MyError,
    models::{BlogPost, NewBlogPost},
    utils::generate_unique_slug,
};

#[get("/")]
async fn get_posts(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client = pool.get().await.map_err(MyError::PoolError)?;

    let posts = db::get_posts(&client).await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{slug}")]
async fn get_post(slug: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let slug_value = slug.into_inner();

    let client = pool.get().await.map_err(MyError::PoolError)?;

    let post = db::get_post_by_slug(&client, &slug_value).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[post("/")]
async fn add_post(
    post: web::Json<NewBlogPost>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let post_data: NewBlogPost = post.into_inner();

    if let Err(e) = post_data.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let client = pool.get().await.unwrap();
    let slug = generate_unique_slug(&client, &post_data.title)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let new_post = db::add_post(&client, BlogPost::new(post_data, slug)).await?;

    Ok(HttpResponse::Ok().json(new_post))
}

#[put("/{slug}")]
async fn update_post(
    slug: web::Path<String>,
    post: web::Json<NewBlogPost>,
    pool: web::Data<Pool>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let slug_value = slug.into_inner();
    let post_data: NewBlogPost = post.into_inner();

    if let Err(e) = post_data.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let client = pool.get().await.unwrap();

    // let need_update_slug = query.get("update_slug").map_or(false, |v| v == "1");

    let new_slug = match query.get("update_slug").map(|v| v.as_str()) {
        Some("1") => Some(generate_unique_slug(&client, &post_data.title).await?),
        _ => None,
    };

    let mut existing_post = db::get_post_by_slug(&client, &slug_value).await?;

    existing_post.update_from(post_data, new_slug);

    let updated_post = db::update_post(&client, &existing_post).await?;

    Ok(HttpResponse::Ok().json(updated_post))
}

pub fn config() -> Scope {
    web::scope("/post")
        .service(get_posts)
        .service(get_post)
        .service(add_post)
        .service(update_post)
}
