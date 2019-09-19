pub mod town_input;
pub mod town_shop;

use quicksilver::prelude::*;
use pathfinding::prelude::{astar, absdiff};
use crate::gui::{
    sprites::*,
    z::{Z_TILE_SHADOW, Z_TEXTURE, Z_VISITOR}
};
pub use paddlers_shared_lib::game_mechanics::town::TileIndex;
use crate::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
pub (crate) use paddlers_shared_lib::game_mechanics::town::TownTileType as TileType;
use paddlers_shared_lib::game_mechanics::town::TileState as TileStateEx;
pub type TileState = TileStateEx<specs::Entity>;

#[derive(Debug)]
pub struct Town {
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


    pub fn render(&self, window: &mut Window, sprites: &mut Sprites, tick: u32, unit_length: f32) -> Result<()> {
        let d = unit_length;

        for (x, col) in self.map.0.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                match tile {
                    TileType::EMPTY | TileType::BUILDING(_) => {
                        // println!("Empty {} {}", x, y);
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Grass))),
                            Transform::IDENTITY,
                            Z_TEXTURE
                        );
                    }

                    TileType::LANE => {
                        // println!("Lane {} {}", x, y);
                        let shifted = ((tick / 10) % (d as u32)) as i32;
                        let t = Transform::translate((shifted,0));
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                            Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                            t,
                            Z_TEXTURE
                        );
                        // XXX: Hack only works for basic map
                        if x == 0 {
                            let x = -1;
                            window.draw_ex(
                                &Rectangle::new((d * x as f32, d * y as f32), (d, d)),
                                Img(&sprites.index(SpriteIndex::Simple(SingleSprite::Water))),
                                t,
                                Z_TEXTURE
                            );
                        }
                        let grass_top_img = &sprites.index(SpriteIndex::Simple(SingleSprite::GrassTop));
                        let h = d / grass_top_img.area().width() * grass_top_img.area().height();
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32 + d - h), (d, h)),
                            Img(grass_top_img),
                            Transform::IDENTITY,
                            Z_VISITOR + 1 // This should be above visitors
                        );
                        let grass_bot_img = &sprites.index(SpriteIndex::Simple(SingleSprite::GrassBot));
                        let h = d / grass_bot_img.area().width() * grass_bot_img.area().height();
                        window.draw_ex(
                            &Rectangle::new((d * x as f32, d * y as f32), (d, h)),
                            Img(grass_bot_img),
                            Transform::IDENTITY,
                            Z_TEXTURE + 1
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub fn shadow_rectified_circle(&self, window: &mut Window, center: impl Into<Vector>, radius: f32){
        let tile = self.tile(center);
        for (x,y) in self.tiles_in_rectified_circle(tile, radius) {
            self.shadow_tile(window, (x,y));
        }
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

    pub fn tile(&self, pos: impl Into<Vector>) -> (usize, usize) {
        Self::find_tile(pos, self.ul)
    }
    pub fn find_tile(pos: impl Into<Vector>, ul: f32) -> (usize, usize) {
        let v = pos.into();
        let x = (v.x / ul) as usize;
        let y = (v.y / ul) as usize;
        (x,y)
    }
    pub fn tile_area(&self, i: TileIndex) -> Rectangle {
        Rectangle::new(Vector::from((i.0 as u32, i.1 as u32)) * self.ul, (self.ul, self.ul))
    }
    pub fn next_tile_in_direction(&self, pos: impl Into<Vector>, dir: impl Into<Vector>) -> (usize, usize) {
        let dir = dir.into();
        let mut pos = pos.into();
        if dir.x < 0.0 {
            pos.x = (pos.x / self.ul).floor() * self.ul;       
        } else if dir.x > 0.0 {
            pos.x = (pos.x / self.ul).ceil() * self.ul;       
        }
        if dir.y < 0.0 {
            pos.y = (pos.y / self.ul).floor() * self.ul;       
        } else if dir.y > 0.0 {
            pos.y = (pos.y / self.ul).ceil() * self.ul;       
        }
        Self::find_tile(pos, self.ul)
    }
    
    pub fn tile_state(&self, i: TileIndex) -> Option<&TileState> {
        self.state.get(&i)
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

    #[inline]
    /// Range should be in unit lengths
    fn are_tiles_in_range(a: (usize, usize), b: (usize, usize), range: f32) -> bool {
        let dx = ( a.0.max(b.0) - a.0.min(b.0) ) as f32;
        let dy = ( a.1.max(b.1) - a.1.min(b.1) ) as f32;
        dx*dx + dy*dy <= range*range
    }

    fn shadow_tile(&self, window: &mut Window, coordinates: (usize,usize)) {
        let shadow_col = Color { r: 1.0, g: 1.0, b: 0.5, a: 0.3 };
        let (x,y) = coordinates;
        let pos = (x as f32 * self.ul, y as f32 * self.ul);
        let size = (self.ul, self.ul);
        let area = Rectangle::new(pos, size);
        window.draw_ex(
            &area,
            Col(shadow_col),
            Transform::IDENTITY, 
            Z_TILE_SHADOW,
        );
    }

    pub fn available_tasks(&self, i: TileIndex) -> Vec<TaskType> {
        match self.map[i] {
            TileType::BUILDING(b) => {
                match b {
                    BuildingType::BundlingStation 
                        => vec![TaskType::GatherSticks],
                    BuildingType::SawMill 
                        => vec![TaskType::ChopTree],
                    _ => vec![],
                }
            }
            TileType::EMPTY => {
                vec![TaskType::Idle]
            }
            TileType::LANE => {
                vec![]
            }
        }
    }
    
    pub fn shortest_path(&self, s: TileIndex, t: TileIndex) -> Option<(Vec<TileIndex>, u32)> {
        let successors = |v: &TileIndex| self.successors(*v);
        let success = |v: &TileIndex| *v == t;
        let heuristic = |v: &TileIndex| (absdiff(v.0, t.0) + absdiff(v.1, t.1)) as u32;
        astar(&s, successors, heuristic, success)
    }

    fn is_buildable(&self, index: TileIndex) -> bool {
        let maybe_tile = self.map.tile_type(index);
        if maybe_tile.is_none() {
            return false;
        }
        maybe_tile.unwrap().is_buildable()
    }
    fn is_walkable(&self, index: TileIndex) -> bool {
        let maybe_tile = self.map.tile_type(index);
        if maybe_tile.is_none() {
            return false;
        }
        maybe_tile.unwrap().is_walkable()
    }

    fn successors(&self, index: TileIndex) -> Vec<(TileIndex, u32)> {
        let (x, y) = index;
        let mut nbrs = vec![];

        if x+1 < X {
            nbrs.push((x+1, y));
        }
        if y+1 < Y {
            nbrs.push((x, y+1));
        }
        if x > 0 {
            nbrs.push((x-1, y));
        }
        if y > 0 {
            nbrs.push((x, y-1));
        }
        nbrs.into_iter()
            .filter( |idx| self.is_walkable(*idx))
            .map(    |idx| (idx, 1))
            .collect()
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
