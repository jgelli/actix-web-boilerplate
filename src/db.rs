use deadpool_postgres::{Config, Pool, Runtime};
use std::env;
use tokio_postgres::NoTls;

pub async fn create_pool() -> Pool {
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");

    let mut cfg = Config::new();
    cfg.dbname = Some(db_name);
    cfg.user = Some(db_user);
    cfg.password = Some(db_password);
    cfg.host = Some(db_host);

    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
