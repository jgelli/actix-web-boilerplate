use super::error::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }
}

pub fn generate_jwt(username: &str, secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(4))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims::new(username.to_owned(), expiration as usize);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data)
}
