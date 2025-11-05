use axum_extra::headers::authorization::Basic;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{db::Object, models::User, schema::users};

pub async fn authenticate(
    db: &mut Object,
    credentials: &Basic,
) -> Result<Option<User>, crate::Error> {
    let user = users::table
        .select(User::as_select())
        .filter(users::name.eq(credentials.username()))
        .first(db)
        .await
        .optional()?;

    if let Some(user) = &user {
        let hash = User::hash(credentials.password().to_string(), &user.salt);
        if hash != user.password {
            return Ok(None);
        }
    }

    Ok(user)
}
