pub use super::village_meta::VillageMetaInfo;
use paddle::*;
use paddlers_shared_lib::game_mechanics::map::MAP_STREAM_AREA_W;

/// Holds data for a segment of the global map
/// and keeps a rendered copy of the rivers cached
pub struct MapSegment {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub streams: Vec<Vec<(f32, f32)>>,
    pub water_mesh: AbstractMesh,
}

impl MapSegment {
    pub fn new(x: i32, y: i32, w: i32, h: i32, streams: Vec<Vec<(f32, f32)>>) -> Self {
        MapSegment {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
            streams,
            water_mesh: AbstractMesh::new(),
        }
    }
    pub fn base_shape(&self) -> Rectangle {
        Rectangle::new((self.x, self.y), (self.w, self.h))
    }
    pub fn scaled_base_shape(&self) -> Rectangle {
        let scaling = super::GlobalMap::unit_length();
        Rectangle::new(
            (self.x as f32 * scaling, self.y as f32 * scaling),
            (self.w as f32 * scaling, self.h as f32 * scaling),
        )
    }
    pub fn is_visible(&self, view: Rectangle) -> bool {
        let overlap = MAP_STREAM_AREA_W * super::GlobalMap::unit_length();
        self.x <= view.x() + view.width() + overlap && self.x + self.w + overlap >= view.x()
    }
}
