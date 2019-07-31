use crate::models::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct BuildingState {
    pub capacity: usize,
    pub entity_count: usize,
}
impl BuildingState {
    pub fn new(bt: BuildingType, entity_count: usize) -> Self {
        BuildingState {
            capacity: bt.capacity(),
            entity_count: entity_count,
        }
    }
}

impl BuildingType {
    pub fn capacity(&self) -> usize {
        match self {
            BuildingType::BundlingStation => 2,
            _ => 0,
        }
    }
}