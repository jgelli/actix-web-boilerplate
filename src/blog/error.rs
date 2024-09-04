// https://github.com/actix/examples/blob/master/basics/error-handling/src/main.rs
use derive_more::{Display, From};
pub type Result<T> = core::result::Result<T, Error>;

use deadpool_postgres::PoolError;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, From, Display)]
pub enum Error {
    NotFound(String),
    UnexpectResult(String),

    // Database
    #[from]
    PGError(PGError),

    #[from]
    PGMError(PGMError),

    #[from]
    PoolError(PoolError),
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
            Error::PGError(ref err) => {
                println!("PGERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
            Error::PGMError(ref err) => {
                println!("PGMERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
            Error::PoolError(ref err) => {
                println!("POOLERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
