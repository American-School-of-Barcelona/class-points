use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{self, State},
    http::StatusCode,
};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::{Basic, Bearer},
    },
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use serde_json::json;

use crate::{
    App, auth,
    error::AsStatus,
    models::{
        User,
        users::{ROLE_ADMIN, ROLE_STUDENT},
    },
    schema,
};

pub async fn list(State(state): State<Arc<App>>) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = &mut state.db().await;

    let list = schema::users::table
        .select(User::as_select())
        .filter(schema::users::role.ne(ROLE_ADMIN))
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
    pub email: String,
    pub password: String,
}

#[debug_handler]
pub async fn register(
    State(state): State<Arc<App>>,
    Json(payload): Json<Register>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let user = User::new(payload.name, payload.email, payload.password, ROLE_STUDENT);
    let email = state.verifications.registration(user).await.status()?;

    Ok(Json(json!({
        "email": email
    })))
}

#[derive(Deserialize)]
pub struct Code {
    code: u16,
}

#[debug_handler]
pub async fn verify(
    State(state): State<Arc<App>>,
    extract::Query(Code { code }): extract::Query<Code>,
) -> Result<Json<User>, StatusCode> {
    let user = state
        .verifications
        .verify(code)
        .await
        .ok_or(StatusCode::FORBIDDEN)?;
    let connection = &mut state.db().await;
    diesel::insert_into(schema::users::table)
        .values(user.clone())
        .execute(connection)
        .await
        .status()?;

    Ok(axum::Json(user))
}

#[debug_handler]
pub async fn login(
    State(state): State<Arc<App>>,
    TypedHeader(Authorization(credentials)): TypedHeader<Authorization<Basic>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let connection = &mut state.db().await;
    let token = auth::login(connection, &credentials)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(json!({
        "token": token
    })))
}

#[debug_handler]
pub async fn authenticated(
    State(state): State<Arc<App>>,
    TypedHeader(Authorization(credentials)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<User>, StatusCode> {
    let connection = &mut state.db().await;
    let user = auth::authenticate(credentials, connection).await?;

    Ok(Json(user))
}
