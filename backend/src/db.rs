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

pub async fn admin(db: &mut Object) -> Result<(), crate::Error> {
    let password = env::var("ADMIN_PASSWORD")?;
    let email = env::var("ADMIN_EMAIL")?;
    diesel::insert_into(schema::users::table)
        .values(User::new(
            String::from("admin"),
            email,
            password,
            ROLE_ADMIN,
        ))
        .on_conflict_do_nothing()
        .execute(db)
        .await?;

    Ok(())
}

pub async fn init() -> Result<Pool, crate::Error> {
    dotenv().ok();
    let manager = AsyncDieselConnectionManager::<Connection>::new(env::var("DB")?);
    let pool = deadpool::Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    admin(&mut pool.get().await?).await?;
    Ok(pool)
}
