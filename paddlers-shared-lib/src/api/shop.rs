use crate::api::keys::VillageKey;
use crate::models::*;
use serde::{Serialize, Deserialize};

pub struct Price(pub Vec<(ResourceType, i64)>);
pub trait Cost {
    fn cost(&self) -> Vec<(ResourceType, i64)>;
    fn price(&self) -> Price { Price(self.cost()) }
}

impl Cost for BuildingType {
    fn cost(&self) -> Vec<(ResourceType, i64)> {
        match self {
            BuildingType::BlueFlowers 
                => vec![(ResourceType::Feathers, 20)],
            BuildingType::RedFlowers
                => vec![
                    (ResourceType::Feathers, 50),
                    (ResourceType::Sticks, 5),
                ],
            BuildingType::Tree
                => vec![
                    (ResourceType::Sticks, 10),
                ],
            BuildingType::BundlingStation
                => vec![
                    (ResourceType::Feathers, 20),
                    (ResourceType::Sticks, 5),
                ],
            BuildingType::SawMill
                => vec![
                    (ResourceType::Feathers, 20),
                    (ResourceType::Sticks, 20),
                ],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuildingPurchase {
    pub village: VillageKey,
    pub building_type: BuildingType,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuildingDeletion {
    pub village: VillageKey,
    pub x: usize,
    pub y: usize,
}
