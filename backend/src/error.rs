use axum::http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("sqlite connection error: {0}")]
    Sqlite(#[from] diesel::ConnectionError),

    #[error("database error: {0}")]
    Diesel(#[from] diesel::result::Error),
}

pub trait AsStatus<T> {
    fn status(self) -> Result<T, StatusCode>;
}

impl<T, E> AsStatus<T> for Result<T, E> {
    fn status(self) -> Result<T, StatusCode> {
        self.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
