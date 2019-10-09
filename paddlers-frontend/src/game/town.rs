pub mod town_input;
pub mod town_shop;
pub mod town_render;
pub mod task_factory;
pub mod tiling;
pub mod path_finding;

use quicksilver::prelude::*;
use crate::gui::{
    sprites::*,
    z::{Z_TILE_SHADOW, Z_TEXTURE, Z_VISITOR}
};
use crate::game::state::*;
pub use paddlers_shared_lib::game_mechanics::town::TileIndex;
use paddlers_shared_lib::prelude::*;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
pub (crate) use paddlers_shared_lib::game_mechanics::town::TownTileType as TileType;
use paddlers_shared_lib::game_mechanics::town::TileState as TileStateEx;
pub type TileState = TileStateEx<specs::Entity>;

#[derive(Debug)]
pub struct Town {
    pub village_key: VillageKey,
    map: TownMap,
    state: TownState<specs::Entity>,
    ul: f32,
    // Could possibly be added to TownState, depends on further developments of the backend.
    pub total_ambience: i64,
}
impl Default for Town {
    fn default() -> Self {
        Town::new(50.0)
    }
}

pub const X: usize = TOWN_X;
const Y: usize = TOWN_Y;
pub const TOWN_RATIO: f32 = X as f32 / Y as f32;

impl Town {
    pub fn new(ul: f32) -> Self {
        let map = TownMap::basic_map();
        Town {
            map: map,
            state: TownState::new(),
            ul: ul,
            total_ambience: 0,
            village_key: current_village(),
        }
    }

    pub fn forest_size(&self) -> usize {
        self.state.forest_size
    }
    pub fn update_forest_size(&mut self, new_score: usize) {
        self.state.forest_size = new_score;
    }
    pub fn forest_usage(&self) -> usize {
        self.state.forest_usage()
    }
    pub fn forest_size_free(&self) -> usize {
        self.state.forest_size - self.state.forest_usage()
    }
    pub fn ambience(&self) -> i64 {
        self.total_ambience
    }
    pub fn distance_to_lane(&self, i: TileIndex) -> f32 {
        self.map.distance_to_lane(i)
    }

    #[allow(dead_code)]
    pub fn grow_forest(&mut self, add_score: usize) {
        self.state.forest_size += add_score;
    }
    /// Call this when a worker begins a task which has an effect on the Town's state
    pub fn add_stateful_task(&mut self, task: TaskType) -> PadlResult<()> {
        self.state.register_task_begin(task).map_err(PadlError::from)
    }
    /// Call this when a worker ends a task which has an effect on the Town's state
    pub fn remove_stateful_task(&mut self, task: TaskType) -> PadlResult<()> {
        self.state.register_task_end(task).map_err(PadlError::from)
    }

    pub fn get_buildable_tile(&self, pos: impl Into<Vector>) -> Option<TileIndex> {
        let (x,y) = self.tile(pos);
        if self.is_buildable((x,y)) {
            Some((x,y))
        }
        else {
            None
        }
    }
    fn tiles_in_rectified_circle(&self, tile: TileIndex, radius: f32) -> Vec<TileIndex> {
        let r = radius.ceil() as usize;
        let xmin =  tile.0.saturating_sub(r);
        let ymin =  tile.1.saturating_sub(r);
        let xmax = if tile.0 + r + 1 > X { X } else { tile.0 + r + 1 };
        let ymax = if tile.1 + r + 1 > Y { Y } else { tile.1 + r + 1 };
        let mut tiles = vec![];
        for x in xmin .. xmax {
            for y in ymin .. ymax {
                if Self::are_tiles_in_range(tile, (x,y), radius) {
                    tiles.push((x,y));
                }
            }
        }
        tiles
    }
    pub fn lane_in_range(&self, pos: TileIndex, range: f32) -> Vec<TileIndex> {
        self.tiles_in_rectified_circle(pos, range).into_iter().filter( |xy| self.map[*xy] == TileType::LANE ).collect()
    }

    pub fn place_building(&mut self, i: TileIndex, bt: BuildingType, id: specs::Entity) {
        debug_assert!(self.is_buildable(i), "Cannot build here");
        let tile = self.map.tile_type_mut(i);

        debug_assert!(tile.is_some(), "Tile is outside of map");
        *tile.unwrap() = TileType::BUILDING(bt);
        let state = TileState::new_building(id, bt.capacity(), 0);
        self.state.insert(i, state);
    }
    pub fn remove_building(&mut self, i: TileIndex) {
        let tile = self.map.tile_type_mut(i);
        *tile.unwrap() = TileType::EMPTY;
        self.state.remove(&i);
    }
    pub fn building_type(&self, i: TileIndex) -> PadlResult<BuildingType> {
        match self.map.tile_type(i) {
            Some(TileType::BUILDING(b)) => Ok(*b),
            Some(t) => PadlErrorCode::UnexpectedTileType("Some Building", *t).dev(),
            None => PadlErrorCode::MapOverflow(i).dev(),
        }
        
    }

    pub fn add_entity_to_building(&mut self, i: &TileIndex) -> PadlResult<()>{
        match self.state.get_mut(i)
        {
            None => PadlErrorCode::NoStateForTile(*i).dev(),
            Some(s) => s.try_add_entity().map_err(PadlError::from),
        }
    }
    pub fn remove_entity_from_building(&mut self, i: &TileIndex) -> PadlResult<()>{
        match self.state.get_mut(i)
        {
            None => PadlErrorCode::NoStateForTile(*i).dev(),
            Some(s) => s.try_remove_entity().map_err(PadlError::from),
        }
    }
}
