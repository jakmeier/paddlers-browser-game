#![feature(trivial_bounds)]
#![feature(const_in_array_repeat_expressions)]
#![feature(const_fn)]
#[cfg(feature = "sql_db")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "sql_db")]
#[macro_use]
extern crate diesel_migrations;
#[cfg(feature = "enum_utils")]
#[macro_use]
extern crate strum_macros;

#[cfg(feature = "enum_utils")]
pub extern crate strum;

#[macro_use]
pub mod macros;

pub mod api;
pub mod civilization;
pub mod config;
pub mod const_list;
pub mod display;
pub mod generated;
pub mod graphql_types;
pub mod models;
pub mod prelude;
pub mod shared_types;
pub mod story;
pub mod test_data;


#[cfg(feature = "game_mechanics")]
/// Module contains game-logic specification which needs to be shared between the frontend and the game-master (but not the GQL DB interface, or even the specification loader)
pub mod game_mechanics;
/// For specifications that are used by all crates. (E.g. everything required for the central story FSM)
pub mod specification_types;

#[cfg(feature = "sql_db")]
#[allow(unused_imports)]
pub mod schema;

#[cfg(feature = "sql_db")]
pub mod sql_db;

#[cfg(feature = "sql_db")]
pub use sql_db::*;

#[cfg(feature = "user_authentication")]
pub mod user_authentication;
