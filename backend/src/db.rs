use std::env;

use diesel::SqliteConnection;
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, deadpool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use dotenvy::dotenv;

pub type Connection = SyncConnectionWrapper<SqliteConnection>;
pub type Pool = deadpool::Pool<Connection>;
pub type Object = deadpool::Object<Connection>;

pub fn init() -> Result<Pool, crate::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = AsyncDieselConnectionManager::<Connection>::new(url);

    let pool = deadpool::Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    Ok(pool)
}
