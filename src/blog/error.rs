use deadpool_postgres;
use derive_more::{Display, From};
pub type Result<T> = core::result::Result<T, Error>;

use crate::error::DatabaseError;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, From, Display)]
pub enum Error {
    NotFound(String),
    UnexpectResult(String),

    #[from]
    DatabaseError(DatabaseError),
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

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound(ref err) => {
                println!("NOT FOUND: {err}");
                HttpResponse::NotFound().finish()
            }
            Error::UnexpectResult(ref err) => {
                println!("UNEXPECT RESULT: {err}");
                HttpResponse::BadRequest().finish()
            }
            Error::DatabaseError(ref err) => {
                println!("DatabaseError: {err}");
                err.error_response()
            }
        }
    }
}
