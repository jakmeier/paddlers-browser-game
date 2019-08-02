use std::collections::HashMap;
use crate::models::BuildingType;
use crate::game_mechanics::building::*;
use crate::models::*;

pub const TOWN_X: usize = 23;
pub const TOWN_Y: usize = 13;
pub const TOWN_LANE_Y: usize = 6;

#[derive(Debug)]
pub struct TownMap(pub [[TownTileType; TOWN_Y]; TOWN_X]);
pub type TileIndex = (usize, usize);

#[derive(PartialEq, Eq,Clone, Copy, Debug)]
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
    pub fn basic_map() -> TownMap {
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

    pub fn tile_type(&self, index: TileIndex) -> Option<&TownTileType> {
        self.0.get(index.0).and_then(|m| m.get(index.1))
    }
    pub fn tile_type_mut(&mut self, index: TileIndex) -> Option<&mut TownTileType> {
        self.0.get_mut(index.0).and_then(|m| m.get_mut(index.1))
    }
}


impl TownTileType {
    pub fn is_buildable(&self) -> bool {
        match self {
            TownTileType::EMPTY 
                => true,
            TownTileType::BUILDING(_)
            | TownTileType::LANE 
                => false,
        }
    }
    pub fn is_walkable(&self) -> bool {
        match self {
            TownTileType::EMPTY 
            | TownTileType::LANE 
                => true,
            TownTileType::BUILDING(bt)
                => match bt {
                    BuildingType::BundlingStation 
                    | BuildingType::SawMill 
                        => true,
                    _ => false,
                },
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
    pub fn remove(&mut self, tile: &TileIndex) {
        let state = self.tiles.remove(tile);
        self.entity_locations.remove(&state.unwrap().entity);
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
    pub fn register_task_begin(&mut self, task: TaskType) -> Result<(), String> {
        if self.has_supply_for_additional_worker(task) {
            let required = task.required_forest_size();
            self.forest_usage += required;
            Ok(())
        } else {
            Err("Not enough supplies".to_owned())
        }
    }
    pub fn register_task_end(&mut self, task: TaskType) -> Result<(), String>  {
        let required = task.required_forest_size();
        if self.forest_usage >= required {
            self.forest_usage -= required;
            Ok(())
        } else {
            Err("Invalid forest usage state".to_owned())
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
            }
        }   
    }
    pub fn try_add_entity(&mut self) -> Result<(), String> {
        if self.building_state.capacity > self.building_state.entity_count {
            self.building_state.entity_count += 1;
            Ok(())
        } else {
            Err("No space in building".to_owned())
        }
    }
    pub fn try_remove_entity(&mut self) -> Result<(), String> {
        if 0 < self.building_state.entity_count {
            self.building_state.entity_count -= 1;
            Ok(())
        } else {
            Err("No entity to remove".to_owned())
        }
    }
}


use std::ops::{Index, IndexMut};
impl Index<TileIndex> for TownMap {
    type Output = TownTileType;

    fn index(&self, idx: TileIndex)-> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}
impl IndexMut<TileIndex> for TownMap {
    fn index_mut(&mut self, idx: TileIndex)-> &mut Self::Output {
        &mut self.0[idx.0][idx.1]
    }
}

pub fn distance2(a: TileIndex, b: TileIndex) -> f32 {
    let x = (a.0 as i32 - b.0 as i32) as f32;
    let y = (a.1 as i32 - b.1 as i32) as f32;
    x * x + y * y
}
