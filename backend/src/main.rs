use axum::{Json, Router, extract::State, routing::get};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub use error::Error;

use crate::{db::create_post, models::Post};

pub struct App {
    db: Mutex<SqliteConnection>,
}

impl App {
    pub fn init() -> Result<Arc<App>, crate::Error> {
        let mut connection = db::init()?;
        db::create_post(&mut connection, "hello", "world");
        eprintln!("Inserted!");

        Ok(Arc::new(Self {
            db: Mutex::new(connection),
        }))
    }
}

pub async fn handler(State(state): State<Arc<App>>) -> Json<Post> {
    use self::schema::posts::dsl::*;
    let mut db = state.db.lock().await;
    let results = posts
        .limit(1)
        .select(Post::as_select())
        .load(&mut *db)
        .expect("Error loading posts");

    Json(results[0].clone())
}

#[tokio::main]
async fn main() -> Result<(), crate::Error> {
    let state = App::init()?;

    let app = Router::new().route("/", get(handler)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    eprintln!("server: listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
