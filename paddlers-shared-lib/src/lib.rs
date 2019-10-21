#![feature(trivial_bounds)]
#[cfg(feature = "sql_db")]
#[macro_use] extern crate diesel;
#[cfg(feature = "sql_db")]
#[macro_use] extern crate diesel_migrations;
#[cfg(feature = "enum_utils")] 
#[macro_use] extern crate strum_macros;

#[cfg(feature = "enum_utils")] 
pub extern crate strum;

pub mod models;
pub mod display;
pub mod api;
pub mod prelude;
pub mod graphql_types;
pub mod config;

#[cfg(feature = "game_mechanics")] 
pub mod game_mechanics;

#[cfg(feature = "sql_db")]
pub mod schema;

#[cfg(feature = "sql_db")] 
pub mod sql_db;

#[cfg(feature = "sql_db")]
pub use sql_db::*; 

#[cfg(feature = "user_authentication")]
pub mod user_authentication;

/// The default ID Type for referencing objects across the Paddlers services.
pub type PadlId = i64;