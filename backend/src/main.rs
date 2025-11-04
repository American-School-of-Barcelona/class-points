#![warn(clippy::pedantic)]

use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod schema;
pub use error::Error;

use crate::db::Pool;

pub struct App {
    pool: Pool,
}

impl App {
    pub async fn init() -> Result<Arc<App>, crate::Error> {
        let pool: Pool = db::init()?;
        Ok(Arc::new(Self { pool }))
    }

    pub async fn db(&self) -> db::Object {
        self.pool.get().await.unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), crate::Error> {
    let state = App::init().await?;

    let app = Router::new()
        .route("/students/new", post(handlers::students::new))
        .route("/students/list", get(handlers::students::list))
        .route("/points/{id}", get(handlers::points::amount))
        .route("/points/{id}/modify", post(handlers::points::modify))
        .route("/points/{id}/history", get(handlers::points::history))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    eprintln!("server: listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
