use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::students;

#[derive(Queryable, Selectable, Serialize, Debug, Clone)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[diesel(table_name = students)]
pub struct New {
    pub name: String,
}
