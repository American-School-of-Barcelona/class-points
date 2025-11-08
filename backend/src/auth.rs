use axum::http::StatusCode;
use axum_extra::headers::authorization::Bearer;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use eyre::Result;
use serde::Deserialize;

use crate::{
    db::Object,
    error::AsStatus,
    models::{self, User},
    schema::{self, users},
};

pub mod jwt;

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub issuer: String,
}

pub async fn login(db: &mut Object, credentials: Login) -> Option<String> {
    let user = users::table
        .select(User::as_select())
        .filter(users::name.eq(credentials.username))
        .first(db)
        .await
        .optional()
        .ok()??;

    let allowed = User::verify(&user.password, &credentials.password).ok()?;
    if !allowed {
        return None;
    }

    jwt::generate(user.id, &credentials.issuer).ok()
}

pub async fn authenticate(
    credentials: Bearer,
    db: &mut Object,
) -> Result<models::User, StatusCode> {
    let claims = jwt::bearer(credentials)?;
    let user: models::User = schema::users::table
        .select(models::User::as_select())
        .filter(schema::users::id.eq(claims.sub))
        .first(db)
        .await
        .status()?;

    Ok(user)
}
