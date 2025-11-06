use axum::http::StatusCode;
use axum_extra::headers::authorization::{Basic, Bearer};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    db::Object,
    error::AsStatus,
    models::{self, User},
    schema::{self, users},
};

pub mod jwt;

pub async fn login(db: &mut Object, credentials: &Basic) -> Option<String> {
    let user = users::table
        .select(User::as_select())
        .filter(users::name.eq(credentials.username()))
        .first(db)
        .await
        .optional()
        .ok()??;

    let hash = User::hash(credentials.password().to_string(), &user.salt);
    if hash != user.password {
        return None;
    }

    jwt::generate(user.id).ok()
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
