use axum_extra::headers::authorization::Basic;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use sha2::Digest;

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
        let hashed =
            sha2::Sha256::digest([credentials.password().as_bytes(), &user.salt].concat()).to_vec();

        if hashed != user.password {
            return Ok(None);
        }
    }

    Ok(user)
}
