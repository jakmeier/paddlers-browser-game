pub mod defence;
pub mod town_layout;

pub use defence::{IAttackingHobo, IDefendingTown};
pub use town_layout::{ITownLayout, ITownLayoutMarker, TownLayout};

#[cfg(test)]
mod defence_test;

use crate::game_mechanics::building::*;
use crate::models::BuildingType;
use crate::models::*;
use std::collections::HashMap;

/// Width of town in unit lengths
pub const TOWN_X: usize = 9;
/// Height of town in unit lengths
pub const TOWN_Y: usize = 7;
/// The town Y coordinate where the river flows through
pub const TOWN_LANE_Y: usize = 3;
/// The town X where resting paddlers will wait
pub const TOWN_RESTING_X: usize = 4;
/// How many unhurried visitors can be resting in a town
pub const MAX_VISITOR_QUEUE: usize = 1;

#[derive(Debug)]
pub struct TownMap(pub [[TownTileType; TOWN_Y]; TOWN_X]);
pub type TileIndex = (usize, usize);
pub type TownLayoutIndex = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TownTileType {
    EMPTY,
    BUILDING(BuildingType),
    LANE,
}

#[derive(Default, Debug)]
/// State which is required for validation of actions to enable both frontend and backend to share validation code.
/// The frontend may have this state duplicated in components.
/// State that is only used by the frontend does not belong in here.
pub struct TownState<I: Eq + std::hash::Hash + Clone + Copy + std::fmt::Debug> {
    tiles: HashMap<TileIndex, TileState<I>>,
    entity_locations: HashMap<I, TileIndex>,
    pub forest_size: usize,
    forest_usage: usize,
}
// Note: So far, this has only one use-case which is buildings.
// Likely, refactoring will become necessary to facilitate other states.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TileState<I: Eq + std::hash::Hash + Clone + Copy + std::fmt::Debug> {
    pub entity: I,
    pub building_state: BuildingState,
}

impl TownMap {
    pub fn new(layout: TownLayout) -> TownMap {
        match layout {
            TownLayout::Basic => {
                let mut map = [[TownTileType::EMPTY; TOWN_Y]; TOWN_X];
                for x in 0..TOWN_X {
                    for y in 0..TOWN_Y {
                        if y == TOWN_LANE_Y {
                            map[x][y] = TownTileType::LANE;
                        }
                    }
                }
                TownMap(map)
            }
        }
    }
    pub fn distance_to_lane(&self, i: TileIndex) -> f32 {
        // Only works for simple map!
        let d = (i.1 as f32 - TOWN_LANE_Y as f32).abs() - 1.0;
        d.max(0.0)
    }

    pub fn tile_type(&self, index: TileIndex) -> Option<&TownTileType> {
        self.0.get(index.0).and_then(|m| m.get(index.1))
    }
    pub fn tile_type_mut(&mut self, index: TileIndex) -> Option<&mut TownTileType> {
        self.0.get_mut(index.0).and_then(|m| m.get_mut(index.1))
    }
    fn iter_town_tiles(&self) -> impl Iterator<Item = &TownTileType> {
        self.0.iter().flat_map(|slice| slice.into_iter())
    }
    pub fn count_tile_type(&self, tile_type: TownTileType) -> usize {
        self.iter_town_tiles().filter(|t| **t == tile_type).count()
    }
    pub fn tiles_with_task(&self, task_type: TaskType) -> Vec<TileIndex> {
        self.iter_town_tiles()
            .enumerate()
            .filter(|(_i, tile)| tile.task_type() == task_type)
            .map(|(i, _tile)| (i / TOWN_Y, i % TOWN_Y))
            .collect()
    }
}

impl TownTileType {
    pub fn is_buildable(&self, bt: BuildingType, index: TileIndex) -> bool {
        match bt {
            BuildingType::Watergate => match self {
                TownTileType::LANE => index.0 == 0 || index.0 == (TOWN_X - 1),
                _ => false,
            },
            _ => match self {
                TownTileType::EMPTY => true,
                TownTileType::BUILDING(_) | TownTileType::LANE => false,
            },
        }
    }
    pub fn is_walkable(&self) -> bool {
        match self {
            TownTileType::EMPTY | TownTileType::LANE => true,
            TownTileType::BUILDING(bt) => match bt {
                BuildingType::BundlingStation
                | BuildingType::SawMill
                | BuildingType::PresentA
                | BuildingType::PresentB
                | BuildingType::RedFlowers
                | BuildingType::BlueFlowers => true,
                _ => false,
            },
        }
    }
    pub fn task_type(&self) -> TaskType {
        match self {
            TownTileType::BUILDING(b) => b.worker_task(),
            TownTileType::EMPTY | TownTileType::LANE => TaskType::Idle,
        }
    }
}

impl<I: Eq + std::hash::Hash + Clone + Copy + std::fmt::Debug> TownState<I> {
    pub fn new() -> Self {
        TownState {
            tiles: HashMap::new(),
            entity_locations: HashMap::new(),
            forest_size: 0,
            forest_usage: 0,
        }
    }
    pub fn forest_usage(&self) -> usize {
        self.forest_usage
    }

    pub fn insert(&mut self, tile: TileIndex, state: TileState<I>) {
        let e = state.entity;
        self.tiles.insert(tile, state);
        self.entity_locations.insert(e, tile);
    }
    pub fn remove(&mut self, tile: &TileIndex) -> TileState<I> {
        let state = self.tiles.remove(tile).unwrap();
        self.entity_locations.remove(&state.entity);
        state
    }
    pub fn get(&self, tile: &TileIndex) -> Option<&TileState<I>> {
        self.tiles.get(tile)
    }
    pub fn get_mut(&mut self, tile: &TileIndex) -> Option<&mut TileState<I>> {
        self.tiles.get_mut(tile)
    }
    pub fn has_supply_for_additional_worker(&self, task: TaskType) -> bool {
        let supply = self.forest_size - self.forest_usage;
        let required = task.required_forest_size();
        supply >= required
    }
    pub fn register_task_begin(&mut self, task: TaskType) -> Result<(), TownError> {
        if self.has_supply_for_additional_worker(task) {
            let required = task.required_forest_size();
            self.forest_usage += required;
            Ok(())
        } else {
            Err(TownError::NotEnoughSupply)
        }
    }
    pub fn register_task_end(&mut self, task: TaskType) -> Result<(), TownError> {
        let required = task.required_forest_size();
        if self.forest_usage >= required {
            self.forest_usage -= required;
            Ok(())
        } else {
            Err(TownError::InvalidState("Forest usage"))
        }
    }
    pub fn count_workers_at(&self, i: &TileIndex) -> usize {
        match self.tiles.get(i) {
            Some(tile_state) => tile_state.building_state.entity_count,
            None => 0,
        }
    }
}

impl<I: Eq + std::hash::Hash + Clone + Copy + std::fmt::Debug> TileState<I> {
    pub fn new_building(e: I, capacity: usize, count: usize) -> Self {
        TileState {
            entity: e,
            building_state: BuildingState {
                capacity: capacity,
                entity_count: count,
            },
        }
    }
    pub fn try_add_entity(&mut self) -> Result<(), TownError> {
        if self.building_state.capacity > self.building_state.entity_count {
            self.building_state.entity_count += 1;
            Ok(())
        } else {
            Err(TownError::BuildingFull)
        }
    }
    pub fn try_remove_entity(&mut self) -> Result<(), TownError> {
        if 0 < self.building_state.entity_count {
            self.building_state.entity_count -= 1;
            Ok(())
        } else {
            Err(TownError::InvalidState("No entity to remove"))
        }
    }
}

use std::ops::{Index, IndexMut};
impl Index<TileIndex> for TownMap {
    type Output = TownTileType;

    fn index(&self, idx: TileIndex) -> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}
impl IndexMut<TileIndex> for TownMap {
    fn index_mut(&mut self, idx: TileIndex) -> &mut Self::Output {
        &mut self.0[idx.0][idx.1]
    }
}

pub fn distance2(a: TileIndex, b: TileIndex) -> f32 {
    let x = (a.0 as i32 - b.0 as i32) as f32;
    let y = (a.1 as i32 - b.1 as i32) as f32;
    x * x + y * y
}

#[derive(Debug)]
pub enum TownError {
    BuildingFull,
    InvalidState(&'static str),
    NotEnoughSupply,
}

impl std::error::Error for TownError {}
impl std::fmt::Display for TownError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TownError::BuildingFull => write!(f, "No space in building"),
            TownError::InvalidState(s) => write!(f, "Invalid state: {}", s),
            TownError::NotEnoughSupply => write!(f, "Not enough supplies"),
        }
    }
}
