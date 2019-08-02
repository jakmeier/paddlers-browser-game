use crate::models::*;

pub trait Attributes {
    fn range(&self) -> Option<f32>;
    fn attack_power(&self) -> Option<i64>;
    fn attacks_per_cycle(&self) -> Option<i64>;
    fn size(&self) -> (usize,usize);
}


impl Attributes for BuildingType {

    fn range(&self) -> Option<f32> {
        match self {
            BuildingType::BlueFlowers => Some(5.0),
            BuildingType::RedFlowers => Some(1.0),
            BuildingType::Tree => None,
            BuildingType::BundlingStation => None,
            BuildingType::SawMill => None,
        }
    }
    fn attack_power(&self) -> Option<i64> {
        match self {
            BuildingType::BlueFlowers => Some(1),
            BuildingType::RedFlowers => Some(3),
            BuildingType::Tree => None,
            BuildingType::BundlingStation => None,
            BuildingType::SawMill => None,
        }
    }
    fn attacks_per_cycle(&self) -> Option<i64>{
        match self {
            BuildingType::BlueFlowers => None,
            BuildingType::RedFlowers => None,
            BuildingType::Tree => None,
            BuildingType::BundlingStation => None,
            BuildingType::SawMill => None,
        }
    }
    fn size(&self) -> (usize,usize) {
        (1,1)
    }
}