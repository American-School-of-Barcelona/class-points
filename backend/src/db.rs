use std::env;

use diesel::SqliteConnection;
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, deadpool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use dotenvy::dotenv;

use crate::{
    models::{User, users::ROLE_ADMIN},
    schema,
};

pub type Connection = SyncConnectionWrapper<SqliteConnection>;
pub type Pool = deadpool::Pool<Connection>;
pub type Object = deadpool::Object<Connection>;

pub async fn init() -> Result<Pool, crate::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::<Connection>::new(url);

    let pool = deadpool::Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    // TODO: Change this in production.
    let result = diesel::insert_into(schema::users::table)
        .values(User::new(
            String::from("admin"),
            String::from("admin"),
            ROLE_ADMIN,
        ))
        .execute(&mut pool.get().await?)
        .await;

    match result {
        Ok(_) => {}
        Err(error) => match error {
            diesel::result::Error::DatabaseError(kind, _)
                if kind == diesel::result::DatabaseErrorKind::UniqueViolation => {}
            _ => return Err(crate::Error::Diesel(error)),
        },
    }

    Ok(pool)
}
