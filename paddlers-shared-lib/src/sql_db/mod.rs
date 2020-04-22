use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod keys;
pub mod sql;

embed_migrations!();
pub use embedded_migrations::run as run_db_migrations;

pub fn establish_connection() -> PgConnection {
    let database_url = get_db_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap_or("postgresql://postgres:password@localhost:5432".to_string())
}
