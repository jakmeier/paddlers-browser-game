use crate::gui::{gui_components::*, input::UiView, sprites::*, ui_state::Now, utils::*};
use crate::prelude::*;
use quicksilver::prelude::*;
use specs::prelude::*;

pub struct MenuButtons {
    ui: UiBox,
    tp: TableTextProvider,
}

impl MenuButtons {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(4, 1, 0.0, 5.0);

        let town_button =
            Self::button_render(SingleSprite::TownButton, SingleSprite::TownButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Town)).with_render_variant(town_button),
        );
        let map_button = Self::button_render(SingleSprite::MapButton, SingleSprite::MapButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Map)).with_render_variant(map_button),
        );

        let atk_button =
            Self::button_render(SingleSprite::AttacksButton, SingleSprite::AttacksButtonHov);
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Attacks))
                .with_render_variant(atk_button),
        );

        let leaderboard_button = Self::button_render(
            SingleSprite::LeaderboardButton,
            SingleSprite::LeaderboardButtonHov,
        );
        ui_box.add(
            UiElement::new(GameEvent::SwitchToView(UiView::Leaderboard))
                .with_render_variant(leaderboard_button),
        );

        let tp = TableTextProvider::new();
        MenuButtons { ui: ui_box, tp }
    }
    fn button_render(normal: SingleSprite, hover: SingleSprite) -> RenderVariant {
        RenderVariant::ImgWithHoverAlternative(SpriteSet::Simple(normal), SpriteSet::Simple(hover))
    }
    pub fn click(&self, mouse: impl Into<Vector>) -> PadlResult<Option<GameEvent>> {
        match self.ui.click(mouse.into())? {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        }
    }
    fn draw(&mut self, window: &mut Window, sprites: &mut Sprites, now: Timestamp, area: &Rectangle) -> PadlResult<()>{
        self.ui.draw(
            window,
            sprites,
            &mut self.tp,
            now,
            area,
        )
    }
}

impl crate::game::Game<'_, '_> {
    pub fn render_buttons(&mut self, window: &mut Window, area: &Rectangle) -> PadlResult<()> {
        let (sprites, buttons, now) = (
            &mut self.sprites,
            &mut self.world.write_resource::<MenuButtons>(),
            self.world.read_resource::<Now>().0,
        );
        buttons.draw(window, sprites, now, area)
    }
    pub fn click_buttons(&mut self, pos: (i32, i32)) {
        let buttons = self.world.fetch::<MenuButtons>();
        let result = buttons.click(pos);
        if let Some(event) = self.check(result).flatten() {
            self.event_pool.send(event).expect("Event pool send failed");
        }
    }
}
