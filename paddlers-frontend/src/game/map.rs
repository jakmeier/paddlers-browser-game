use quicksilver::prelude::*;
use crate::gui::{
    sprites::*,
    z::*,
};

pub struct GlobalMap {

}

impl GlobalMap {
    pub fn new() -> Self {
        GlobalMap {
        }
    }
    pub fn render(&self, window: &mut Window, _sprites: &Sprites, area: &Rectangle) -> Result<()> {
        // TODO: Draw map
        window.draw_ex(
            area,
            Col(Color::GREEN),
            Transform::IDENTITY,
            Z_TEXTURE,
        );
        Ok(())
    }
}