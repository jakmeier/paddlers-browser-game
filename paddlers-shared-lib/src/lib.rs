#![feature(trivial_bounds)]
#[cfg(feature = "sql_db")] 
#[macro_use] extern crate diesel;
#[macro_use] extern crate strum_macros;

pub extern crate strum;

pub mod models;
pub mod api;

#[cfg(feature = "sql_db")]
pub mod schema;

#[cfg(feature = "sql_db")] 
pub mod sql_db;

#[cfg(feature = "sql_db")]
pub use sql_db::*; 
