use actix_web::{get, web, Responder, Scope};

#[get("/")]
async fn get_posts() -> impl Responder {
    "List of blog posts"
}

#[get("/{id}")]
async fn get_post(path: web::Path<(u32,)>) -> impl Responder {
    format!("Details of blog post with id {}", path.0)
}

pub fn config() -> Scope {
    web::scope("/blog").service(get_posts).service(get_post)
}
