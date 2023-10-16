pub const MAP_H: u32 = 11;
pub const MAP_MAX_X: u32 = 1000;
pub const MAP_STREAM_AREA_W: f32 = 5.0;

pub fn map_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)).sqrt()
}
