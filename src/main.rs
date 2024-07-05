mod blog;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/blog").configure(blog::routes::config)))
        .bind("0.0.0.0:8081")?
        .run()
        .await
}
