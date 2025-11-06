use std::cell::LazyCell;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use diesel::prelude::*;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::schema::users;

pub const ARGON2: LazyCell<Argon2> = LazyCell::new(|| Argon2::default());

pub const ROLE_STUDENT: i32 = 0;
pub const ROLE_TEACHER: i32 = 1;
pub const ROLE_ADMIN: i32 = 2;

#[derive(
    Insertable,
    Queryable,
    QueryableByName,
    Selectable,
    Deserialize,
    Serialize,
    Debug,
    Clone,
    Default,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    #[serde(skip_deserializing)]
    #[diesel(skip_insertion)]
    pub id: i32,
    pub name: String,
    #[diesel(skip_insertion)]
    #[serde(skip_deserializing)]
    pub points: i32,
    pub role: i32,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    pub fn verify(hash: &str, password: &str) -> Result<bool, crate::Error> {
        let result = ARGON2.verify_password(password.as_bytes(), &PasswordHash::new(hash)?);

        match result {
            Ok(_) => Ok(true),
            Err(error) => match error {
                argon2::password_hash::Error::Password => Ok(false),
                error => Err(crate::Error::Hash(error)),
            },
        }
    }

    #[must_use]
    pub fn new(
        name: String,
        email: String,
        password: String,
        role: i32,
    ) -> Result<Self, crate::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed = ARGON2.hash_password(password.as_bytes(), &salt)?;

        Ok(Self {
            name,
            email,
            password: hashed.to_string(),
            role,
            ..Default::default()
        })
    }
}
