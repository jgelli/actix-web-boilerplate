mod blog;
mod db;

use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    web, App, HttpServer,
};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .app_data(web::Data::new(pool.clone()))
            .service(blog::routes::config())
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
