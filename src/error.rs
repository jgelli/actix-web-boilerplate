use derive_more::{Display, From};

use deadpool_postgres::PoolError;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, From, Display)]
pub enum DatabaseError {
    #[from]
    PGError(PGError),

    #[from]
    PGMError(PGMError),

    #[from]
    PoolError(PoolError),
}

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DatabaseError::PGError(ref err) => {
                println!("PGERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
            DatabaseError::PGMError(ref err) => {
                println!("PGMERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
            DatabaseError::PoolError(ref err) => {
                println!("POOLERROR: {err}");
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
