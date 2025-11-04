use std::env;

use diesel::{Connection, RunQueryDsl, SelectableHelper, SqliteConnection};
use dotenvy::dotenv;

use crate::models::{NewPost, Post};

pub fn init() -> Result<SqliteConnection, crate::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&url)?;

    Ok(connection)
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .unwrap()
}
