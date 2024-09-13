// src/users/error.rs
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct UserErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
    HashError(String),
    InvalidCredentials(String),
    UserNotFound(String),
    UserAlreadyExists(String),
    InvalidInput(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Error::HashError(msg) => write!(f, "Hashing error: {}", msg),
            Error::InvalidCredentials(msg) => write!(f, "{}", msg),
            Error::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            Error::UserAlreadyExists(msg) => write!(f, "User already exists: {}", msg),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let error_message = match self {
            Error::DatabaseError(msg) => msg.clone(),
            Error::HashError(msg) => msg.clone(),
            Error::InvalidCredentials(msg) => msg.clone(),
            Error::UserNotFound(msg) => msg.clone(),
            Error::UserAlreadyExists(msg) => msg.clone(),
            Error::InvalidInput(msg) => msg.clone(),
        };

        HttpResponse::BadRequest().json(UserErrorResponse {
            message: error_message,
        })
    }
}

// Convert external lib errors in custom errors
impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::DatabaseError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        Error::HashError(err.to_string())
    }
}
