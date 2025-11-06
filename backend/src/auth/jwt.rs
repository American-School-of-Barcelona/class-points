use std::env;

use axum::http::StatusCode;
use axum_extra::headers::authorization::Bearer;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

use crate::{auth::jwt, error::AsStatus};

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn generate(id: i32) -> Result<String, crate::Error> {
    let secret = env::var("SECRET_KEY")?;
    let exp = (Utc::now() + Duration::hours(128)).timestamp() as usize;
    let claims = Claims { sub: id, exp };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_base64_secret(&secret)?,
    )?;

    Ok(token)
}

pub fn verify(token: &str) -> Result<TokenData<Claims>, crate::Error> {
    let secret = env::var("SECRET_KEY")?;
    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_base64_secret(&secret)?,
        &Validation::new(Algorithm::HS256),
    )?)
}

#[inline]
pub fn bearer(credentials: Bearer) -> Result<Claims, StatusCode> {
    Ok(jwt::verify(credentials.token()).status()?.claims)
}
