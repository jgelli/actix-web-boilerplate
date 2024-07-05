use actix_web::{get, web, Responder};

#[get("/")]
async fn get_posts() -> impl Responder {
    "List of blog posts"
}

#[get("/{id}")]
async fn get_post(path: web::Path<(u32,)>) -> impl Responder {
    format!("Details of blog post with id {}", path.0)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts);
    cfg.service(get_post);
}
