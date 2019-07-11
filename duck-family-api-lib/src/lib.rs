use serde::{Serialize, Deserialize};
pub use duck_family_db_lib::models::*;
pub use BuildingType;

#[derive(Serialize, Deserialize)]
pub struct BuildingPurchase {
    pub building_type: BuildingType,
    pub x: usize,
    pub y: usize,
}