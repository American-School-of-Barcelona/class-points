use std::sync::Arc;

use axum::{Json, debug_handler, extract::State, http::StatusCode};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use serde_json::json;

use crate::{
    App,
    error::AsStatus,
    models::{User, users::ROLE_STUDENT},
    schema,
};

pub async fn list(State(state): State<Arc<App>>) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = &mut state.db().await;

    let list = schema::users::table
        .select(User::as_select())
        .filter(schema::users::role.eq(0))
        .load(connection)
        .await
        .status()?;

    Ok(Json(json!({
        "students": list
    })))
}

#[derive(Deserialize)]
pub struct Register {
    pub name: String,
    pub password: String,
}

#[debug_handler]
pub async fn register(
    State(state): State<Arc<App>>,
    Json(payload): Json<Register>,
) -> Result<Json<User>, StatusCode> {
    let connection = &mut state.db().await;
    let user = diesel::insert_into(schema::users::table)
        .values(User::new(payload.name, payload.password, ROLE_STUDENT))
        .returning(User::as_returning())
        .get_result(connection)
        .await
        .status()?;

    Ok(Json(user))
}
