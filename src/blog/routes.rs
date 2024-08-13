use actix_web::{get, post, web, Error, HttpResponse, Responder, Scope};
use deadpool_postgres::Pool;
use validator::Validate;

use super::{
    db,
    errors::MyError,
    models::{BlogPost, NewBlogPost},
    utils::generate_unique_slug,
};

// use crate::blog::utils::generate_unique_slug;

#[get("/")]
async fn get_posts(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client = pool.get().await.map_err(MyError::PoolError)?;

    let posts = db::get_posts(&client).await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{id}")]
async fn get_post(path: web::Path<(u32,)>) -> impl Responder {
    format!("Details of blog post with id {}", path.0)
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
    println!("{}", &slug);

    let new_post = db::add_post(&client, BlogPost::new(post_data, slug)).await?;
    println!("{:?}", &new_post);

    Ok(HttpResponse::Ok().json(new_post))
}

pub fn config() -> Scope {
    web::scope("/post")
        .service(get_posts)
        .service(get_post)
        .service(add_post)
}
