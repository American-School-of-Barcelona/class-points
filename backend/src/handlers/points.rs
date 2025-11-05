use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use serde_json::json;

use crate::{App, error::AsStatus, models, schema};

#[axum::debug_handler]
pub async fn history(
    State(state): State<Arc<App>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = &mut state.db().await;

    use schema::records::dsl::*;
    let results = records
        .select(&models::Record::as_select())
        .filter(student.eq(id))
        .load(connection)
        .await
        .status()?;

    Ok(Json(json!({
        "history": results
    })))
}

pub async fn amount(
    State(state): State<Arc<App>>,
    Path(id): Path<i32>,
) -> Result<Json<models::Student>, StatusCode> {
    let connection = &mut state.db().await;

    let student = schema::students::table
        .select(models::Student::as_select())
        .filter(schema::students::id.eq(id))
        .first(connection)
        .await
        .status()?;

    Ok(Json(student))
}

#[derive(Deserialize)]
pub struct Modify {
    amount: i32,
    reason: String,
    set: bool,
}

#[debug_handler]
pub async fn modify(
    State(state): State<Arc<App>>,
    Path(id): Path<i32>,
    Json(payload): Json<Modify>,
) -> Result<Json<models::Student>, StatusCode> {
    let connection = &mut state.db().await;

    let update = diesel::update(schema::students::table.filter(schema::students::id.eq(id)));
    let (student, delta) = if payload.set {
        let old: models::Student = schema::students::table
            .filter(schema::students::id.eq(id))
            .first(connection)
            .await
            .status()?;

        let result = update
            .set(schema::students::points.eq(payload.amount))
            .returning(models::Student::as_returning())
            .get_result(connection)
            .await
            .status()?;

        let delta = result.points - old.points;
        (result, delta)
    } else {
        let result = update
            .set(schema::students::points.eq(schema::students::points + payload.amount))
            .returning(models::Student::as_returning())
            .get_result(connection)
            .await
            .status()?;

        (result, payload.amount)
    };

    diesel::insert_into(schema::records::table)
        .values(&models::Record {
            points: student.points,
            change: delta,
            reason: payload.reason,
            date: chrono::Utc::now().to_rfc3339(),
            student: id,
        })
        .execute(connection)
        .await
        .status()?;

    Ok(Json(student))
}
