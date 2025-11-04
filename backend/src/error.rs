#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("sqlite connection error: {0}")]
    Sqlite(#[from] diesel::ConnectionError),

    #[error("database error: {0}")]
    Diesel(#[from] diesel::result::Error),
}
