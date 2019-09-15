use quicksilver::prelude::*;
use quicksilver::graphics::{Mesh};
pub use super::village_meta::VillageMetaInfo;

/// Holds data for a segment of the global map
/// and keeps a rendered copy of the rivers cached
pub struct MapSegment {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub streams: Vec<Vec<(f32,f32)>>,
    pub water_mesh: Mesh,
    pub scaling: f32,
}

impl MapSegment {
    pub fn new(x: i32, y: i32, w: i32, h: i32, streams: Vec<Vec<(f32,f32)>>) -> Self {
        let scaling = 1.0;
        MapSegment {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
            streams,
            water_mesh: Mesh::new(),
            scaling,
        }
    }
    pub fn base_shape(&self) -> Rectangle {
        Rectangle::new(
            (self.x,self.y),
            (self.w, self.h),
        )
    }
    pub fn scaled_base_shape(&self) -> Rectangle {
        let scaling = self.scaling;
        Rectangle::new(
            (self.x as f32 * scaling, self.y as f32 * scaling),
            (self.w as f32 * scaling, self.h as f32 * scaling),
        )
    }
    pub fn apply_scaling(&mut self, r: f32) {
        if self.scaling != r {
            self.scaling = r;
            self.tesselate_rivers();
        }
    }
    pub fn is_visible(&self, view: Rectangle) -> bool {
        self.x <= view.x() + view.width() 
        && self.x + self.w >= view.x()
    }
}