#![feature(trivial_bounds)]
#[cfg(feature = "sql_db")] 
#[macro_use] extern crate diesel;
#[cfg(feature = "enum_utils")] 
#[macro_use] extern crate strum_macros;

#[cfg(feature = "enum_utils")] 
pub extern crate strum;

pub mod models;
pub mod api;
pub mod prelude;

#[cfg(feature = "game_mechanics")] 
pub mod game_mechanics;

#[cfg(feature = "sql_db")]
pub mod schema;

#[cfg(feature = "sql_db")] 
pub mod sql_db;

#[cfg(feature = "sql_db")]
pub use sql_db::*; 
