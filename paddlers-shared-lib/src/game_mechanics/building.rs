use crate::story::story_state::StoryState;
use crate::{
    civilization::{CivilizationPerk, CivilizationPerks},
    models::*,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Part of [TileState](crate::game_mechanics::town::TileState) for validation state used similarly in backend and frontend.
///
/// See also description of [TownState](crate::game_mechanics::town::TownState)
pub struct BuildingState {
    pub typ: BuildingType,
    entity_count: u32,
    level: u32,
}
impl BuildingState {
    pub fn new(typ: BuildingType, level: i32, entity_count: usize) -> Self {
        debug_assert!(level > 0);
        BuildingState {
            typ,
            entity_count: entity_count as u32,
            level: level as u32,
        }
    }
    /// How many entities can be inside
    pub const fn capacity(&self) -> usize {
        self.typ.capacity()
    }
    /// How many entities are inside right now
    pub const fn entity_count(&self) -> usize {
        self.entity_count as usize
    }
    pub fn add_entity(&mut self) {
        self.entity_count += 1;
    }
    pub fn remove_entity(&mut self) {
        self.entity_count += 1;
    }
    pub fn visitor_queue_capacity(&self) -> usize {
        match self.typ {
            BuildingType::Watergate => self.level as usize,
            _ => 0,
        }
    }
    pub fn contained_queued_visitors(&self) -> usize {
        match self.typ {
            BuildingType::Watergate => self.entity_count as usize,
            _ => 0,
        }
    }
}

impl BuildingType {
    pub const fn capacity(&self) -> usize {
        match self {
            BuildingType::BundlingStation => 2,
            BuildingType::SawMill => 1,
            BuildingType::Watergate => 6,
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
    pub fn player_can_build(
        &self,
        karma: i64,
        story_state: StoryState,
        civ: CivilizationPerks,
    ) -> bool {
        match self {
            BuildingType::BlueFlowers => karma >= 1,
            BuildingType::BundlingStation => karma >= 20,
            BuildingType::PresentA => karma >= 250,
            BuildingType::PresentB => karma >= 750,
            BuildingType::RedFlowers => karma >= 450,
            BuildingType::SawMill => karma >= 150,
            BuildingType::Temple => story_state == StoryState::ServantAccepted,
            BuildingType::Tree => karma >= 1,
            BuildingType::SingleNest => civ.has(CivilizationPerk::NestBuilding),
            BuildingType::TripleNest => civ.has(CivilizationPerk::TripleNestBuilding),
            BuildingType::Watergate => story_state == StoryState::BuildingWatergate,
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
            BuildingType::Watergate,
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
            BuildingType::Watergate => false,
        }
    }
    pub fn worker_task(&self) -> TaskType {
        match self {
            BuildingType::BundlingStation => TaskType::GatherSticks,
            BuildingType::SawMill => TaskType::ChopTree,
            _ => TaskType::Idle,
        }
    }
}
