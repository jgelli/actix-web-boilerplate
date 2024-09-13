use super::{error::Error, models::User};
use bcrypt::verify;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub async fn get_user_by_username(client: &Client, username: &str) -> Result<User, Error> {
    let stmt = include_str!("sql/get_user_by_username.sql");
    let stmt = client.prepare(&stmt).await?;

    let row_opt = client.query_opt(&stmt, &[&username]).await?;

    let row = match row_opt {
        Some(row) => row,
        None => return Err(Error::UserNotFound("User not found".to_string())),
    };

    let user = User::from_row_ref(&row)?;
    Ok(user)
}

async fn verify_password(row: &Row, provided_password: &str) -> Result<(), Error> {
    let stored_password: String = row.get("password");

    match verify(provided_password, &stored_password) {
        Ok(is_valid) if is_valid => Ok(()),
        Ok(_) => Err(Error::InvalidCredentials(
            "Invalid username or password".to_string(),
        )),
        Err(e) => Err(Error::HashError(format!(
            "Password verification failed: {}",
            e
        ))),
    }
}
