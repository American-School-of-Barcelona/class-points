#![warn(clippy::pedantic)]

use axum::{
    Router,
    routing::{get, patch, post},
};
use eyre::Result;
use std::{sync::Arc, time::Duration};

pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod schema;
pub use error::Error;
pub mod auth;
pub mod email;
pub mod verification;

use crate::{db::Pool, verification::Verifications};

pub struct App {
    pool: Pool,
    verifications: Verifications,
}

impl App {
    pub async fn init() -> Result<Arc<App>, crate::Error> {
        let pool: Pool = db::init().await?;
        Ok(Arc::new(Self {
            pool,
            verifications: Verifications::new(),
        }))
    }

    pub async fn db(&self) -> db::Object {
        self.pool.get().await.unwrap()
    }

    pub async fn worker(app: Arc<Self>) {
        loop {
            app.verifications.prune().await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), crate::Error> {
    dotenvy::dotenv()?;
    let state = App::init().await?;
    let worker = tokio::spawn(App::worker(state.clone()));

    let app = Router::new()
        .route("/users/register", post(handlers::users::register))
        .route("/users/verify", post(handlers::users::verify))
        .route("/users/login", post(handlers::users::login))
        .route("/users/authenticated", get(handlers::users::authenticated))
        .route("/users/list", get(handlers::users::list))
        .route("/points/{id}", get(handlers::points::amount))
        .route("/points/{id}/modify", patch(handlers::points::modify))
        .route("/points/{id}/history", get(handlers::points::history))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    eprintln!("server: listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    worker.abort();
    Ok(())
}
