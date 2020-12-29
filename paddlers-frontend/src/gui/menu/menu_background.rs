use super::*;
use crate::net::NetMsg;
use crate::prelude::*;
use crate::{
    game::game_event_manager::game_event,
    gui::{gui_components::*, input::UiView, sprites::*, ui_state::Now, utils::*},
};
use crate::{game::toplevel::Signal, gui::decoration::*};
use paddle::{DisplayArea, NutsCheck};
use specs::prelude::*;

pub(crate) struct MenuBackgroundFrame {
    ui: UiBox,
    tp: TableTextProvider,
    reports_to_collect: usize,
    mouse: PointerTracker,
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
            mouse: PointerTracker::new(),
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
    fn left_click(&mut self, state: &mut Game, pos: Vector) {
        let result = match self.ui.click(pos) {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            game_event(event);
        }
    }
}

impl Frame for MenuBackgroundFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::OUTER_MENU_AREA_W;
    const HEIGHT: u32 = crate::resolution::OUTER_MENU_AREA_H;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.tp.reset();
        state.draw_menu_background(window).nuts_check();
        let button_area = crate::gui::menu::nav_area();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui.draw(
            window,
            sprites,
            &mut self.tp,
            now,
            &button_area,
            self.mouse.pos(),
            Z_UI_MENU,
        );
        self.tp.finish_draw();
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        if let PointerEvent(PointerEventType::PrimaryClick, pos) = event {
            self.left_click(state, pos)
        }
    }
}

impl Game {
    fn draw_menu_background(&mut self, window: &mut DisplayArea) -> PadlResult<()> {
        let area = crate::gui::menu::menu_box_area();

        // Menu Box Background
        window.draw_ex(&area, Col(LIGHT_GREEN), Transform::IDENTITY, Z_MENU_BOX);

        draw_leaf_border(
            window,
            &mut self.sprites,
            &area,
            LEAVES_BORDER_W,
            LEAVES_BORDER_H,
        );

        let area = duck_step_area();
        draw_duck_step_line(
            window,
            &mut self.sprites,
            Vector::new(area.x() - LEAVES_BORDER_W * 0.5, area.pos.y),
            area.x() + area.width() + LEAVES_BORDER_W * 0.5,
            DUCK_STEPS_H,
        );

        Ok(())
    }
}
