use actix_web::{post, web, HttpResponse, Scope};
use deadpool_postgres::Pool;
use serde::Deserialize;
use validator::Validate;

use super::{error::Error, jwt::generate_jwt};
use crate::user::{db, models::LoginUser};

#[derive(Deserialize, Validate)]
struct LoginParams {
    #[validate(length(min = 3))]
    username: String,
    #[validate(length(min = 6))]
    password: String,
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool>,
    jwt_secret: web::Data<String>,
    login_params: web::Json<LoginParams>,
) -> Result<HttpResponse, Error> {
    if let Err(e) = login_params.validate() {
        return Ok(HttpResponse::BadRequest().json(e.to_string()));
    }

    let client = pool.get().await?;

    let login_user = LoginUser {
        username: login_params.username.clone(),
        password: login_params.password.clone(),
    };

    let user = db::get_user_by_username(&client, &login_user.username).await?;

    let token = generate_jwt(&user.username, &jwt_secret)?;

    Ok(HttpResponse::Ok().json(token))
}


pub fn config() -> Scope {
    web::scope("/auth").service(login)
}
