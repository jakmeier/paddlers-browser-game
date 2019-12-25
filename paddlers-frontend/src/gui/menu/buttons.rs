use quicksilver::prelude::*;
use specs::prelude::*;
use crate::prelude::*;
use crate::gui::{
    sprites::*,
    gui_components::*,
    input::UiView,
    ui_state::Now,
    utils::*,
};

pub struct MenuButtons {
    ui: UiBox,
}

impl MenuButtons {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(4, 1, 0.0, 5.0);
        ui_box.add(UiElement::empty());

        let town_button = Self::button_render(SingleSprite::TownButton, SingleSprite::TownButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Town))
                .with_render_variant(town_button)
        );
        
        let map_button = Self::button_render(SingleSprite::MapButton, SingleSprite::MapButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Map))
                .with_render_variant(map_button)
        );

        let atk_button = Self::button_render(SingleSprite::AttacksButton, SingleSprite::AttacksButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Attacks))
                .with_render_variant(atk_button)
        );
        
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
    pub fn click(&self, mouse: impl Into<Vector>) -> PadlResult<Option<GameEvent>> {
        match self.ui.click(mouse.into())? {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None)
        }
    }
}

impl crate::game::Game<'_, '_> {
    pub fn render_buttons(&mut self, window: &mut Window, area: &Rectangle) -> Result<()> {
        let (sprites, mut buttons) = (&mut self.sprites, self.world.write_resource::<MenuButtons>());
        buttons.ui.draw(window, sprites.as_mut().unwrap(), self.world.read_resource::<Now>().0, area)
    }
}