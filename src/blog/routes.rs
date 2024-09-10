use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

use super::{
    db,
    error::Error,
    models::{BlogPost, NewBlogPost, UpdateBlogPost},
    utils::generate_unique_slug,
};

#[derive(Deserialize, Validate)]
pub struct ListParams {
    last_id: Option<i32>,
    limit: Option<i64>,
    active: Option<bool>, //TODO: validate permission

    #[validate(length(min = 3, message = "Title must be at least 3 characters long to filter posts."))]
    title: Option<String>,
}

#[get("")]
async fn get_posts(
    pool: web::Data<Pool>,
    query: web::Query<ListParams>,
) -> Result<HttpResponse, Error> {

    if let Err(e) = query.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let last_id = query.last_id.unwrap_or(i32::MAX);
    let limit = query.limit.unwrap_or(10).min(25);
    let active = query.active.unwrap_or(true); //TODO: validate permission to see inactive posts

    let client = pool.get().await?;

    let posts = db::get_posts(&client, last_id, limit, active, &query.title).await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{slug}")]
async fn get_post(slug: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let slug_value = slug.into_inner();

    let client = pool.get().await?;

    let post = db::get_post_by_slug(&client, &slug_value).await?; //TODO: validate permission to see inactive post

    Ok(HttpResponse::Ok().json(post))
}

#[post("")]
async fn add_post(
    post: web::Json<NewBlogPost>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let post_data: NewBlogPost = post.into_inner();

    if let Err(e) = post_data.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let client = pool.get().await?;
    let slug = generate_unique_slug(&client, &post_data.title).await?;

    let new_post = db::add_post(&client, BlogPost::new(post_data, slug)).await?;

    Ok(HttpResponse::Created().json(new_post))
}

#[put("/{slug}")]
async fn update_post(
    slug: web::Path<String>,
    post: web::Json<UpdateBlogPost>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug_value = slug.into_inner();
    let post_data: UpdateBlogPost = post.into_inner();

    if let Err(e) = post_data.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let client = pool.get().await?;

    let mut existing_post = db::get_post_by_slug(&client, &slug_value).await?;

    let new_slug = match post_data.update_slug {
        true => Some(generate_unique_slug(&client, &post_data.title).await?),
        false => None,
    };

    existing_post.update_from(post_data, new_slug);

    let updated_post = db::update_post(&client, &existing_post).await?;

    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/{slug}")]
async fn delete_post(
    slug: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let slug_value = slug.into_inner();

    let client = pool.get().await?;

    let post = db::get_post_by_slug(&client, &slug_value).await?;

    let deleted_post = db::delete_post(&client, &post.id).await?;

    Ok(HttpResponse::Ok().json(deleted_post))
}

pub fn config() -> Scope {
    web::scope("/post")
        .service(get_posts)
        .service(get_post)
        .service(add_post)
        .service(update_post)
        .service(delete_post)
}
