use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Basic},
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use serde_json::json;

use crate::{
    App, auth,
    error::AsStatus,
    models::{
        self,
        users::{ROLE_ADMIN, ROLE_TEACHER},
    },
    schema,
};

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
) -> Result<Json<models::User>, StatusCode> {
    let connection = &mut state.db().await;

    let student = schema::users::table
        .select(models::User::as_select())
        .filter(schema::users::id.eq(id))
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
    TypedHeader(Authorization(credentials)): TypedHeader<Authorization<Basic>>,
    State(state): State<Arc<App>>,
    Path(id): Path<i32>,
    Json(payload): Json<Modify>,
) -> Result<Json<models::User>, StatusCode> {
    let connection = &mut state.db().await;
    let Some(user) = auth::authenticate(connection, &credentials)
        .await
        .status()?
    else {
        return Err(StatusCode::FORBIDDEN);
    };

    if user.role != ROLE_TEACHER && user.role != ROLE_ADMIN {
        return Err(StatusCode::FORBIDDEN);
    }

    let update = diesel::update(schema::users::table.filter(schema::users::id.eq(id)));
    let (student, delta) = if payload.set {
        let old: i32 = schema::users::table
            .select(schema::users::points)
            .filter(schema::users::id.eq(id))
            .first(connection)
            .await
            .status()?;

        let result = update
            .set(schema::users::points.eq(payload.amount))
            .returning(models::User::as_returning())
            .get_result(connection)
            .await
            .status()?;

        let delta = result.points - old;
        (result, delta)
    } else {
        let result = update
            .set(schema::users::points.eq(schema::users::points + payload.amount))
            .returning(models::User::as_returning())
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
