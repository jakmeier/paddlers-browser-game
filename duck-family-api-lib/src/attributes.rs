use crate::types::*;

pub trait Attributes {
    fn range(&self) -> Option<f32>;
    fn attack_power(&self) -> Option<f32>;
    fn attacks_per_cycle(&self) -> Option<i32>;
}


impl Attributes for BuildingType {

    fn range(&self) -> Option<f32> {
        match self {
            BuildingType::BlueFlowers => Some(5.0),
            BuildingType::RedFlowers => Some(1.0)
        }
    }
    fn attack_power(&self) -> Option<f32> {
        match self {
            BuildingType::BlueFlowers => Some(1.0),
            BuildingType::RedFlowers => Some(3.0)
        }
    }
    fn attacks_per_cycle(&self) -> Option<i32>{
        match self {
            BuildingType::BlueFlowers => None,
            BuildingType::RedFlowers => None
        }
    }
}