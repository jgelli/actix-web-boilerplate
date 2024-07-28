use actix_web::{get, post, web, Error, HttpResponse, Responder, Scope};
use deadpool_postgres::Pool;

use super::db;
use super::models::NewBlogPost;

#[get("/")]
async fn get_posts() -> impl Responder {
    "List of blog posts"
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

    let client = pool.get().await.unwrap();
    let new_post = db::add_post(&client, post_data).await?;

    Ok(HttpResponse::Ok().json(new_post))
}

pub fn config() -> Scope {
    web::scope("/post")
        .service(get_posts)
        .service(get_post)
        .service(add_post)
}
