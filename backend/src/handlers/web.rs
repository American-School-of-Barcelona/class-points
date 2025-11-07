use axum::{http::StatusCode, response::Html};
use axum_extra::response::Css;

use crate::error::AsStatus;

#[inline]
async fn serve(file: &'static str) -> Result<String, StatusCode> {
    tokio::fs::read_to_string(file).await.status()
}

pub async fn register() -> Result<Html<String>, StatusCode> {
    Ok(Html(serve("web/register.html").await?))
}

pub async fn style() -> Result<Css<String>, StatusCode> {
    Ok(Css(serve("web/style.css").await?))
}
