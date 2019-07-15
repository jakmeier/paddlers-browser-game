#![feature(trivial_bounds)]
#[macro_use] extern crate diesel;
pub extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate diesel_derive_enum;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod sql;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

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