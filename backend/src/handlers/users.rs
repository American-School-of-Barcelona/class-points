use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{self, State},
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
    App, auth, email,
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
    let code: u16 = rand::random_range(0..9999);
    email::send(
        &user.email,
        "Email Verification",
        format!("Your code: {code}"),
    )
    .await
    .status()?;

    let email = user.email.clone();
    state.verifications.lock().await.push((code, user));
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
    let mut lock = state.verifications.lock().await;
    let idx = lock
        .iter()
        .position(|x| x.0 == code)
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    let user = lock.remove(idx).1;
    let connection = &mut state.db().await;
    diesel::insert_into(schema::users::table)
        .values(user.clone())
        .execute(connection)
        .await
        .status()?;

    Ok(axum::Json(user))
}

#[debug_handler]
pub async fn authenticated(
    TypedHeader(Authorization(credentials)): TypedHeader<Authorization<Basic>>,
    State(state): State<Arc<App>>,
) -> Result<Json<User>, StatusCode> {
    let connection = &mut state.db().await;
    let authenticated = auth::authenticate(connection, &credentials)
        .await
        .status()?;
    if let Some(user) = authenticated {
        Ok(Json(user))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
