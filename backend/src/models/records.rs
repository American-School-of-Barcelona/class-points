use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::records;

#[derive(
    Queryable, QueryableByName, Selectable, Serialize, Deserialize, Insertable, Debug, Clone,
)]
#[diesel(table_name = records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub change: i32,
    pub reason: String,
    pub date: String,

    #[serde(skip_serializing)]
    pub student: i32,
}
