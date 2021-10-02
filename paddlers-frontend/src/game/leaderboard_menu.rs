use crate::prelude::*;
use crate::{
    game::game_event_manager::game_event,
    gui::{gui_components::*, shapes, sprites::*, ui_state::Now, utils::*, z::Z_UI_MENU},
};
use paddle::{DisplayArea, PointerEvent, PointerEventType, PointerTracker};
use specs::WorldExt;

pub(crate) struct LeaderboardMenuFrame {
    ui: UiBox,
    text_provider: TableTextProvider,
    mouse: PointerTracker,
}

impl LeaderboardMenuFrame {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(1, 5, 0.0, 10.0);
        let tabs = [
            (LeaderboardViewTab::KarmaLeaderboard, SingleSprite::Karma),
            (
                LeaderboardViewTab::IncomingAttacks,
                SingleSprite::DuckShapes,
            ),
        ];
        for (view, img) in &tabs {
            let rend = RenderVariant::ImgWithHoverShape(
                SpriteSet::Simple(*img),
                shapes::SHAPE_FRAME,
                DARK_GREEN,
            );
            ui_box.add(
                UiElement::new(GameEvent::SwitchToView(UiView::Leaderboard(*view)))
                    .with_render_variant(rend),
            );
        }

        LeaderboardMenuFrame {
            ui: ui_box,
            text_provider: TableTextProvider::new(),
            mouse: PointerTracker::new(),
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

impl Frame for LeaderboardMenuFrame {
    type State = Game;
    const WIDTH: u32 = crate::gui::menu::INNER_MENU_AREA_W as u32;
    const HEIGHT: u32 = crate::gui::menu::INNER_MENU_AREA_H as u32;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.text_provider.reset();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui.draw(
            window,
            sprites,
            &mut self.text_provider,
            now,
            &Self::area(),
            self.mouse.pos(),
            Z_UI_MENU,
        );
        self.text_provider.finish_draw();
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        if let PointerEvent(PointerEventType::PrimaryClick, pos) = event {
            self.left_click(state, pos)
        }
    }
}
