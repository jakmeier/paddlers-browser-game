use crate::{api::shop::Price, story::story_state::StoryState};
use crate::{
    civilization::{CivilizationPerk, CivilizationPerks},
    models::*,
};

use super::attributes::Attributes;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Part of [TileState](crate::game_mechanics::town::TileState) for validation state used similarly in backend and frontend.
///
/// See also description of [TownState](crate::game_mechanics::town::TownState)
pub struct BuildingState {
    pub typ: BuildingType,
    entity_count: u16,
    level: u16,
}
impl BuildingState {
    pub fn new(typ: BuildingType, level: i32, entity_count: usize) -> Self {
        debug_assert!(level > 0);
        BuildingState {
            typ,
            entity_count: entity_count as u16,
            level: level as u16,
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
    pub fn set_entity_count(&mut self, n: usize) {
        self.entity_count = n as u16
    }
    pub fn add_entity(&mut self) {
        self.entity_count += 1;
    }
    pub fn remove_entity(&mut self) {
        self.entity_count += 1;
    }
    pub fn visitor_queue_capacity(&self) -> usize {
        self.typ.visitor_queue_capacity(self.level)
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
            BuildingType::BlueFlowers => story_state == StoryState::AllDone,
            BuildingType::BundlingStation => {
                (story_state == StoryState::FirstVisitorWelcomed) || karma >= 20
            }
            BuildingType::PresentA => karma >= 150,
            BuildingType::PresentB => karma >= 250,
            BuildingType::RedFlowers => karma >= 200,
            BuildingType::SawMill => story_state == StoryState::AllDone,
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

impl BuildingType {
    /// No cost means no upgrade available
    pub fn upgrade_cost(&self, current_level: usize) -> Option<Price> {
        match self {
            BuildingType::Watergate => watergate_upgrade_cost(current_level),
            _ => None,
        }
    }
}
fn watergate_upgrade_cost(level: usize) -> Option<Price> {
    match level {
        1 => Some(
            Price::new()
                .with(ResourceType::Feathers, 10)
                .with(ResourceType::Logs, 5),
        ),
        2 => Some(
            Price::new()
                .with(ResourceType::Feathers, 10)
                .with(ResourceType::Logs, 10),
        ),
        3 => Some(
            Price::new()
                .with(ResourceType::Feathers, 50)
                .with(ResourceType::Logs, 20),
        ),
        4 => Some(
            Price::new()
                .with(ResourceType::Feathers, 200)
                .with(ResourceType::Logs, 40),
        ),
        5 => Some(
            Price::new()
                .with(ResourceType::Feathers, 500)
                .with(ResourceType::Logs, 100),
        ),
        _ => None,
    }
}
