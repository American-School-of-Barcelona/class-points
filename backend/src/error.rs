use axum::http::StatusCode;
use diesel::result::DatabaseErrorKind;
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

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("error parsing environment: {0}")]
    DotEnvy(#[from] dotenvy::Error),

    #[error("hashing error: {0}")]
    Hash(#[from] argon2::password_hash::Error),
}

pub trait AsStatus<T> {
    fn status(self) -> Result<T, StatusCode>;
}

impl<T, E: Into<Error>> AsStatus<T> for Result<T, E> {
    fn status(self) -> Result<T, StatusCode> {
        match self {
            Ok(x) => Ok(x),
            Err(error) => {
                let error = error.into();
                eprintln!("server: {error}");
                match error {
                    Error::Jwt(_) => Err(StatusCode::UNAUTHORIZED),
                    Error::Diesel(diesel::result::Error::DatabaseError(kind, _))
                        if kind == DatabaseErrorKind::UniqueViolation =>
                    {
                        Err(StatusCode::CONFLICT)
                    }
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }
}
