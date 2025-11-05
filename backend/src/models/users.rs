use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::schema::users;

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
    pub password: Vec<u8>,
    #[serde(skip_serializing)]
    pub salt: Vec<u8>,
}

impl User {
    #[must_use]
    pub fn new(name: String, password: String, role: i32) -> Self {
        let salt: Vec<u8> = rand::random_iter().take(8).collect();
        let password = [password.as_bytes(), &salt].concat();
        let hashed = sha2::Sha256::digest(password).to_vec();
        Self {
            name,
            password: hashed,
            salt,
            role,
            ..Default::default()
        }
    }
}
