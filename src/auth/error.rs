use derive_more::{Display, From};

use crate::error::DatabaseError;
use crate::user::error::Error as UserError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JwtErrorResponse {
    pub message: String,
}

#[derive(Debug, Display, From)]
pub enum Error {
    TokenCreationError(String),
    InvalidToken(String),
    TokenExpired(String),

    #[from]
    DatabaseError(DatabaseError),

    UserError(UserError),
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::DatabaseError(DatabaseError::from(err))
    }
}

impl From<tokio_pg_mapper::Error> for Error {
    fn from(err: tokio_pg_mapper::Error) -> Self {
        Error::DatabaseError(DatabaseError::from(err))
    }
}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Error::DatabaseError(DatabaseError::from(err))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Error::TokenExpired(err.to_string())
            }
            _ => Error::InvalidToken(err.to_string()),
        }
    }
}

impl From<UserError> for Error {
    fn from(err: UserError) -> Self {
        Error::UserError(UserError::from(err))
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::TokenCreationError(msg)
            | Error::InvalidToken(msg)
            | Error::TokenExpired(msg) => {
                println!("Unauthorized: {msg}");
                HttpResponse::Unauthorized().json(JwtErrorResponse {
                    message: msg.clone(),
                })
            }
            Error::UserError(ref err) => {
                println!("UserError: {err}");
                err.error_response()
            }
            Error::DatabaseError(ref err) => {
                println!("DatabaseError: {err}");
                err.error_response()
            }
        }
    }
}
