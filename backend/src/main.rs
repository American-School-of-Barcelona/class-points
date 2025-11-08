#![warn(clippy::pedantic)]

use axum::{
    Router,
    response::Redirect,
    routing::{get, patch, post},
};
use eyre::Result;
use std::{sync::Arc, time::Duration};
use tower_http::cors::{Any, CorsLayer};

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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/login") }))
        .route("/login", get(handlers::web::login))
        .route("/register", get(handlers::web::register))
        .route("/assets/style.css", get(handlers::web::style))
        .route("/api/users/register", post(handlers::users::register))
        .route("/api/users/verify", post(handlers::users::verify))
        .route("/api/users/login", post(handlers::users::login))
        .route(
            "/api/users/authenticated",
            get(handlers::users::authenticated),
        )
        .route("/api/users/list", get(handlers::users::list))
        .route("/api/points/{id}", get(handlers::points::amount))
        .route("/api/points/{id}/modify", patch(handlers::points::modify))
        .route("/api/points/{id}/history", get(handlers::points::history))
        .with_state(state)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    eprintln!("server: listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    worker.abort();
    Ok(())
}
