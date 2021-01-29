pub mod attacks;
pub mod error;
pub mod hobo;
pub mod keys;
pub mod quests;
pub mod reports;
pub mod shop;
pub mod statistics;
pub mod tasks;

#[cfg(feature = "game_mechanics")]
pub mod story;

use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInitData {
    pub display_name: String,
}
