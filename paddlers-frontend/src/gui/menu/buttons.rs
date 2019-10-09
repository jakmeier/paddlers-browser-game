use quicksilver::prelude::*;
use specs::prelude::*;
use crate::game::specs_resources::Now;
use crate::game::state::*;
use crate::gui::{
    sprites::*,
    gui_components::*,
    input::{UiState, UiView},
    utils::*,
};

pub struct MenuButtons {
    ui: UiBox,
}

#[derive(Debug,Clone, PartialEq, Eq)]
pub enum MenuButtonAction {
    SwitchToView(UiView),
}

impl MenuButtons {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(4, 1, 0.0, 5.0);
        ui_box.add(UiElement::empty());

        let town_button = Self::button_render(SingleSprite::TownButton, SingleSprite::TownButtonHov);
        ui_box.add(
            UiElement::new(MenuButtonAction::SwitchToView(UiView::Town(current_village())))
                .with_render_variant(town_button)
        );
        
        let map_button = Self::button_render(SingleSprite::MapButton, SingleSprite::MapButtonHov);
        ui_box.add(
            UiElement::new(MenuButtonAction::SwitchToView(UiView::Map))
                .with_render_variant(map_button)
        );
        
        ui_box.add(UiElement::empty());
        
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
        if let Some(ClickOutput::MenuButtonAction(action)) = self.ui.click(mouse.into()) {
            match action {
                MenuButtonAction::SwitchToView(v) => {
                    ui_state.set_view(v);
                }
            }
        }
    }
}

impl crate::game::Game<'_, '_> {
    pub fn render_buttons(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let (sprites, mut buttons) = (&mut self.sprites, self.world.write_resource::<MenuButtons>());
        buttons.ui.draw(window, sprites.as_mut().unwrap(), self.world.read_resource::<Now>().0, area)
    }
}