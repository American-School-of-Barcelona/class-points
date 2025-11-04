use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::{
    App,
    models::{self, Student},
    schema,
};

pub async fn handler(
    State(state): State<Arc<App>>,
    Json(payload): Json<models::students::New>,
) -> Result<Json<Student>, StatusCode> {
    let mut connection = state.db().await;

    use schema::students::dsl::*;
    let result = diesel::insert_into(students)
        .values(&models::students::New { name: payload.name })
        .returning(&Student::as_returning())
        .get_result(&mut connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
