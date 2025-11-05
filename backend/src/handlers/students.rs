use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use serde_json::json;

use crate::{
    App,
    error::AsStatus,
    models::{self, Student},
    schema,
};

pub async fn new(
    State(state): State<Arc<App>>,
    Json(payload): Json<models::Student>,
) -> Result<Json<Student>, StatusCode> {
    let connection = &mut state.db().await;

    use schema::students::dsl::*;
    let result = diesel::insert_into(students)
        .values(&models::Student::new(payload.name))
        .returning(&Student::as_returning())
        .get_result(connection)
        .await
        .status()?;

    use schema::records::dsl::*;
    let initial: String = diesel::insert_into(records)
        .values(&models::Record {
            change: 0,
            points: 0,
            reason: String::from("init"),
            date: chrono::Utc::now().to_rfc3339(),
            student: result.id,
        })
        .returning(reason)
        .get_result(connection)
        .await
        .status()?;

    assert_eq!(initial, String::from("init"));

    Ok(Json(result))
}

pub async fn list(State(state): State<Arc<App>>) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = &mut state.db().await;

    let list = schema::students::table
        .select(Student::as_select())
        .load(connection)
        .await
        .status()?;

    Ok(Json(json!({
        "students": list
    })))
}
