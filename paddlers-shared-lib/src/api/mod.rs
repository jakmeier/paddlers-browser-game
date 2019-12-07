pub mod shop;
pub mod tasks;
pub mod statistics;
pub mod keys;
pub mod error;

use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInitData {
    pub display_name: String,
}