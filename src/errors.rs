use actix_web::{http::StatusCode, ResponseError};
use deadpool_postgres::PoolError;
use derive_more;
use tokio_pg_mapper::Error as PostgresMapperError;
use tokio_postgres::error::Error as PostgresError;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum AppError {
    NotFound,
    Conflict,
    PostgresError(PostgresError),
    PoolError(PoolError),
    PostgresMapperError(PostgresMapperError),
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
