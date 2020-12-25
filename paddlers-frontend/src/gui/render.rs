use crate::gui::utils::*;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderVariant,
    /// Size factor is applied when rendering in main window, not in menu
    pub in_game_transformation: f32,
}
impl Renderable {
    pub fn new(kind: RenderVariant) -> Self {
        Renderable {
            kind,
            in_game_transformation: std::f32::NAN,
        }
    }
    pub fn new_transformed(kind: RenderVariant, in_game_transformation: f32) -> Self {
        Renderable {
            kind,
            in_game_transformation,
        }
    }
}
