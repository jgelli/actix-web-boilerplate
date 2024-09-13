mod auth;
mod blog;
mod db;
mod error;
mod user;

use actix_web::{web, App, HttpServer};
use std::env;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::create_pool().await;

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .service(blog::routes::config())
            .service(auth::routes::config())
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
