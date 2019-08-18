use quicksilver::prelude::*;
use specs::prelude::*;
use crate::gui::{
    sprites::*,
    gui_components::*,
    input::UiState,
};

pub struct MenuButtons {
    ui: UiBox<MenuButtonAction>,
}

#[derive(Debug,Clone)]
enum MenuButtonAction {
    ToggleView
}

impl MenuButtons {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(5, 1, 0.0, 5.0);
        ui_box.add_empty();
        ui_box.add_empty();
        ui_box.add_empty();
        ui_box.add_empty();
        ui_box.add(SpriteSet::Simple(SingleSprite::MapButton), MenuButtonAction::ToggleView);
        MenuButtons {
            ui: ui_box
        }
    }
    pub fn click(&self, mouse: impl Into<Vector>, ui_state: &mut UiState) {
        if let Some(action) = self.ui.click(mouse) {
            match action {
                MenuButtonAction::ToggleView => {
                    ui_state.toggle_view();
                }
            }
        }
    }
}

impl crate::game::Game<'_, '_> {
    pub fn render_buttons(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let (sprites, mut buttons) = (&mut self.sprites, self.world.write_resource::<MenuButtons>());
        buttons.ui.draw(window, sprites, area)
    }
}