use specs::prelude::*;
use quicksilver::prelude::*;
use crate::gui::input::Clickable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
/// A position on the map view
/// Coordinates are in map view (unscaled) but zerp-indexed
pub struct MapPosition {
    pub x: f32,
    pub y: f32,
}

impl MapPosition {
    /// Directly dd coordinates as provided by the server
    pub fn new(coordinates: (i32, i32)) -> Self {
        MapPosition {
            x: coordinates.0 as f32 - 1.0,
            y: coordinates.1 as f32 - 1.0,
        }
    }
}

pub fn map_position_lookup<'a>(
    mouse_pos: Vector,
    entities: Entities<'a>,
    position: ReadStorage<'a, MapPosition>,
    clickable: ReadStorage<'a, Clickable>,
) -> Option<Entity> {

    for (e, pos, _) in (&entities, &position, &clickable).join() {
        let area = Rectangle::new((pos.x, pos.y), (1.0, 1.0));
        if mouse_pos.overlaps_rectangle(&area) {
           return Some(e);
        }
    }
    None
}
