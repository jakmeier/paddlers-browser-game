use crate::models::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Part of [TileState](crate::game_mechanics::town::TileState) for validation state used similarly in backend and frontend.
/// 
/// See also description of [TownState](crate::game_mechanics::town::TownState)
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
            BuildingType::SawMill => 1,
            _ => 0,
        }
    }
}

impl BuildingType {
    /// Experience gained when collection the building as a reward
    pub fn reward_exp(&self) -> Option<i32> {
        match self {
            BuildingType::PresentA => Some(10),
            BuildingType::PresentB => Some(30),
            _ => None,
        }
    }
}