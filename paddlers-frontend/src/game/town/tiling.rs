use quicksilver::prelude::*;
use super::{Town, TileState, TileIndex};

impl Town {
   

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

    #[inline]
    /// Range should be in unit lengths
    pub (super) fn are_tiles_in_range(a: (usize, usize), b: (usize, usize), range: f32) -> bool {
        let dx = ( a.0.max(b.0) - a.0.min(b.0) ) as f32;
        let dy = ( a.1.max(b.1) - a.1.min(b.1) ) as f32;
        dx*dx + dy*dy <= range*range
    }

    pub (super) fn is_buildable(&self, index: TileIndex) -> bool {
        let maybe_tile = self.map.tile_type(index);
        if maybe_tile.is_none() {
            return false;
        }
        maybe_tile.unwrap().is_buildable()
    }
    pub (super) fn is_walkable(&self, index: TileIndex) -> bool {
        let maybe_tile = self.map.tile_type(index);
        if maybe_tile.is_none() {
            return false;
        }
        maybe_tile.unwrap().is_walkable()
    }
}
