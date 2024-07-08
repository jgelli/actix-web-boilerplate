mod blog;
mod db;

use deadpool_postgres::Pool;

use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;

async fn db_test(pool: web::Data<Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let row = client.query_one("SELECT 1 + 1", &[]).await.unwrap();
    let sum: i32 = row.get(0);
    format!("1 + 1 = {}", sum)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(blog::routes::config())
            .route("/dbtest", web::get().to(db_test))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
