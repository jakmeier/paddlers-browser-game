pub mod attacks;
pub mod error;
pub mod keys;
pub mod shop;
pub mod statistics;
pub mod tasks;

use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInitData {
    pub display_name: String,
}