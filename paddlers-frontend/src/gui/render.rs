use crate::gui::utils::*;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub kind: RenderVariant,
    /// Size factor is applied when rendering in main window, not in menu
    pub in_game_transformation: f32,
    pub on_selection: Option<RenderVariant>,
}
impl Renderable {
    pub fn new(kind: RenderVariant) -> Self {
        Renderable {
            kind,
            in_game_transformation: std::f32::NAN,
            on_selection: None,
        }
    }
    pub fn new_transformed(kind: RenderVariant, in_game_transformation: f32) -> Self {
        Renderable {
            kind,
            in_game_transformation,
            on_selection: None,
        }
    }
    pub fn with_on_selection(mut self, kind: Option<RenderVariant>) -> Self {
        self.on_selection = kind;
        self
    }
}
