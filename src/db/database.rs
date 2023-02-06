use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
//const DB_CONN_STR: &'static str = "/home/alex/Projects/Rust/rust-diesel-crud/sqlite.db";

use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
