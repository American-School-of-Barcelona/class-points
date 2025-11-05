use axum::http::StatusCode;
use diesel_async::pooled_connection::deadpool;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("sqlite connection error: {0}")]
    Sqlite(#[from] diesel::ConnectionError),

    #[error("database error: {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("pool error: {0}")]
    Pool(#[from] deadpool::PoolError),

    #[error("missing environment variable: {0}")]
    Env(#[from] std::env::VarError),

    #[error("smtp error: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),

    #[error("email address error: {0}")]
    Address(#[from] lettre::address::AddressError),

    #[error("email content error: {0}")]
    Lettre(#[from] lettre::error::Error),
}

pub trait AsStatus<T> {
    fn status(self) -> Result<T, StatusCode>;
}

impl<T, E> AsStatus<T> for Result<T, E> {
    fn status(self) -> Result<T, StatusCode> {
        self.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
