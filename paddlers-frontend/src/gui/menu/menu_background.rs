use crate::gui::{gui_components::*, input::UiView, sprites::*, ui_state::Now, utils::*};
use crate::game::toplevel::Signal;
use crate::net::NetMsg;
use crate::prelude::*;
use paddle::{DisplayArea, NutsCheck};
use specs::prelude::*;

pub(crate) struct MenuBackgroundFrame {
    ui: UiBox,
    tp: TableTextProvider,
    reports_to_collect: usize,
}

impl MenuBackgroundFrame {
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
                VisitorViewTab::Letters,
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
            reports_to_collect: 0,
        }
    }
    fn button_render(normal: SingleSprite, hover: SingleSprite) -> RenderVariant {
        RenderVariant::ImgWithHoverAlternative(SpriteSet::Simple(normal), SpriteSet::Simple(hover))
    }
    fn update_notifications(&mut self) {
        self.ui
            .update_notifications(Some(vec![0, 0, self.reports_to_collect, 0]));
    }
    pub fn network_message(&mut self, _state: &mut Game, msg: &NetMsg) {
        match msg {
            NetMsg::Reports(data) => {
                let new_reports = data.village.reports.len();
                self.reports_to_collect += new_reports;
                self.update_notifications();
            }
            _ => {}
        }
    }
    pub fn signal(&mut self, _state: &mut Game, msg: &Signal) {
        match msg {
            Signal::NewReportCount(n) => {
                self.reports_to_collect = *n;
                self.update_notifications();
            }
            _ => {}
        }
    }
}

impl Frame for MenuBackgroundFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::MENU_AREA_W;
    const HEIGHT: u32 = crate::resolution::MENU_AREA_H;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.tp.reset();
        state.draw_menu_background(window).nuts_check();
        let button_area = state.button_area();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui.draw(
            window,
            sprites,
            &mut self.tp,
            now,
            &button_area,
            state.mouse.pos(),
        );
        self.tp.finish_draw();
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let result = match self.ui.click(pos.into()) {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            nuts::publish(event);
        }
    }
}
