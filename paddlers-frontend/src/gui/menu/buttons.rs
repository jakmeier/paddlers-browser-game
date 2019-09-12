use quicksilver::prelude::*;
use specs::prelude::*;
use crate::gui::{
    sprites::*,
    gui_components::*,
    input::UiState,
    utils::*,
};

pub struct MenuButtons {
    ui: UiBox<MenuButtonAction>,
}

#[derive(Debug,Clone)]
enum MenuButtonAction {
    // ToMapView,
    // ToTownView,
    ToggleView
}

impl MenuButtons {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(4, 1, 0.0, 5.0);
        ui_box.add_empty();

        let map_button = Self::button_render(SingleSprite::TownButton, SingleSprite::TownButtonHov);
        ui_box.add_with_render_variant(map_button, MenuButtonAction::ToggleView);
        
        let map_button = Self::button_render(SingleSprite::MapButton, SingleSprite::MapButtonHov);
        ui_box.add_with_render_variant(map_button, MenuButtonAction::ToggleView);
        
        ui_box.add_empty();
        
        MenuButtons {
            ui: ui_box
        }
    }
    fn button_render(normal: SingleSprite, hover: SingleSprite) -> RenderVariant {
        RenderVariant::ImgWithHoverAlternative(
            SpriteSet::Simple(normal),
            SpriteSet::Simple(hover),
        )
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