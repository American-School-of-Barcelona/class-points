use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::students;

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
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    #[serde(skip_deserializing)]
    #[diesel(skip_insertion)]
    pub id: i32,
    pub name: String,
    #[diesel(skip_insertion)]
    #[serde(skip_deserializing)]
    pub points: i32,
}

impl Student {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
