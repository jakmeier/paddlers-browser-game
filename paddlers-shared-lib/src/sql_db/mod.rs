use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod sql;

embed_migrations!();

pub fn establish_connection() -> PgConnection {

    let database_url = get_db_url();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_db_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres@localhost:5432".to_string())
}

pub fn initiliaze_db_if_env_set(conn: &PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    if env::var("DATABASE_INIT").is_ok() {
        embedded_migrations::run(conn)?;
    }
    Ok(())
}