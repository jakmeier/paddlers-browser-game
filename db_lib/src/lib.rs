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
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres@localhost:5432".to_string());
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}