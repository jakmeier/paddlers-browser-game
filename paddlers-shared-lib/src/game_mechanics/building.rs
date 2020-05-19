use crate::models::*;
use crate::story::story_state::StoryState;

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

impl BuildingType {
    /// Definition of which buildings are available to a player
    pub fn player_can_build(&self, karma: i64, story_state: StoryState) -> bool {
        match self {
            BuildingType::BlueFlowers => karma >= 1,
            BuildingType::BundlingStation => karma >= 1,
            BuildingType::PresentA => karma >= 200,
            BuildingType::PresentB => karma >= 2000,
            BuildingType::RedFlowers => karma >= 1000,
            BuildingType::SawMill => karma >= 100,
            BuildingType::Temple => story_state == StoryState::ServantAccepted,
            BuildingType::Tree => karma >= 1,
            BuildingType::SingleNest => karma >= 500,
            BuildingType::TripleNest => karma >= 3000,
        }
    }
    /// Buildings that may be available at the default shop, regardless of player restrictions
    pub fn default_shop_buildings<'a>() -> impl Iterator<Item = &'a BuildingType> {
        [
            BuildingType::BlueFlowers,
            BuildingType::BundlingStation,
            BuildingType::PresentA,
            BuildingType::PresentB,
            BuildingType::RedFlowers,
            BuildingType::SawMill,
            BuildingType::Tree,
            BuildingType::Temple,
            BuildingType::SingleNest,
            BuildingType::TripleNest,
        ]
        .iter()
    }
    /// Based on the building type only, some buildings cannot be deleted, as defined in this function.
    pub fn can_be_deleted(&self) -> bool {
        match self {
            BuildingType::BlueFlowers => true,
            BuildingType::BundlingStation => true,
            BuildingType::PresentA => false,
            BuildingType::PresentB => false,
            BuildingType::RedFlowers => true,
            BuildingType::SawMill => true,
            BuildingType::Temple => false,
            BuildingType::Tree => true,
            BuildingType::SingleNest => false, // false for now, to avoid problems with associated hobos
            BuildingType::TripleNest => false, // false for now, to avoid problems with associated hobos
        }
    }
}
