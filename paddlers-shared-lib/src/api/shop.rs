use crate::api::keys::VillageKey;
use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Price(pub Vec<(ResourceType, i64)>);
pub trait Cost {
    fn cost(&self) -> Vec<(ResourceType, i64)>;
    fn price(&self) -> Price {
        Price(self.cost())
    }
}

impl Cost for BuildingType {
    fn cost(&self) -> Vec<(ResourceType, i64)> {
        match self {
            BuildingType::BlueFlowers => vec![(ResourceType::Feathers, 20)],
            BuildingType::RedFlowers => {
                vec![(ResourceType::Feathers, 100), (ResourceType::Sticks, 20)]
            }
            BuildingType::Tree => vec![(ResourceType::Sticks, 10)],
            BuildingType::BundlingStation => vec![(ResourceType::Feathers, 20)],
            BuildingType::SawMill => vec![(ResourceType::Feathers, 20), (ResourceType::Sticks, 20)],
            BuildingType::PresentA => vec![(ResourceType::Feathers, 100)],
            BuildingType::PresentB => vec![(ResourceType::Sticks, 50), (ResourceType::Logs, 50)],
            BuildingType::Temple => vec![],
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ProphetPurchase {
    pub village: VillageKey,
}
