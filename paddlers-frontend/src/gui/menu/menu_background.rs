use crate::gui::{gui_components::*, input::UiView, sprites::*, ui_state::Now, utils::*};
use crate::prelude::*;
use specs::prelude::*;

use crate::init::quicksilver_integration::Signal;
use crate::view::*;
use core::marker::PhantomData;
use quicksilver::prelude::Window;

pub(crate) struct MenuBackgroundFrame<'a, 'b> {
    ui: UiBox,
    tp: TableTextProvider,
    _phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> MenuBackgroundFrame<'a, 'b> {
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
            UiElement::new(GameEvent::SwitchToView(UiView::Visitors(
                VisitorViewTab::IncomingAttacks,
            )))
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
        MenuBackgroundFrame {
            ui: ui_box,
            tp,
            _phantom: Default::default(),
        }
    }
    fn button_render(normal: SingleSprite, hover: SingleSprite) -> RenderVariant {
        RenderVariant::ImgWithHoverAlternative(SpriteSet::Simple(normal), SpriteSet::Simple(hover))
    }
}

impl<'a, 'b> Frame for MenuBackgroundFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    type Signal = Signal;

    fn draw(&mut self, state: &mut Self::State, window: &mut Window) -> Result<(), Self::Error> {
        state.draw_menu_background(window)?;
        let button_area = state.button_area();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui
            .draw(window, sprites, &mut self.tp, now, &button_area)
    }
    fn left_click(
        &mut self,
        state: &mut Self::State,
        pos: (i32, i32),
        _signals: &mut ExperimentalSignalChannel,
    ) -> Result<(), Self::Error> {
        let result = match self.ui.click(pos.into())? {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            state
                .event_pool
                .send(event)
                .expect("Event pool send failed");
        }
        Ok(())
    }
}
