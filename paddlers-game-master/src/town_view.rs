use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;
use crate::db::{DB};

pub struct TownView {
    map: TownMap,
}

impl TownView {
    pub (crate) fn load_village(db: &DB, _village_id: i64) -> Self {
        let mut map = TownMap::basic_map();

        let buildings = db.buildings();
        for b in buildings {
            map[(b.x as usize, b.y as usize)] = TownTileType::BUILDING(b.building_type);
        }

        TownView {
            map: map,
        }
    }

    pub (crate) fn path_walkable(&self, start: TileIndex, end: TileIndex) -> bool {
        let (x,y) = start;
        let mut dy = 0;
        let mut dx = 0;
        if x != end.0 {
            if y != end.1 {
                //println!("Path must be a straight line but was {:?}->{:?}", start, end);
                return false;
            } 
            dx = if end.0 < x { -1 } else { 1 };
        } else {
            dy = if end.1 < y { -1 } else { 1 };
        }
        let mut pos = start;
        while pos != end {
            if !self.map[pos].is_walkable() {
                return false;
            }
            pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize)
        }
        true
    }
}