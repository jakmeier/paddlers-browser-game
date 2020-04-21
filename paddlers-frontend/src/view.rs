mod floating_text;
mod frame;
mod text_node;
pub mod text_pool;
use crate::game::story::select_dialogue_scene;
pub use floating_text::*;
pub use frame::*;
use paddlers_shared_lib::story::story_state::StoryState;
pub use text_node::*;

use crate::game::Game;
use crate::gui::input::UiView;
use crate::gui::ui_state::UiState;

impl Game<'_, '_> {
    pub fn switch_view(&mut self, view: UiView) {
        let ui: &mut UiState = &mut *self.world.fetch_mut();
        ui.leave_view();
        ui.current_view = view;
    }
    pub fn toggle_view(&mut self) {
        let ui: shred::Fetch<UiState> = self.world.fetch();
        let next = match (*ui).current_view {
            UiView::Map => UiView::Town,
            UiView::Town => UiView::Attacks,
            UiView::Attacks => UiView::Leaderboard,
            UiView::Leaderboard => UiView::Map,
            UiView::Dialogue => return,
        };
        std::mem::drop(ui);

        self.switch_view(next);
    }
}

pub fn entry_view(story_state: StoryState) -> UiView {
    if select_dialogue_scene(story_state).is_some() {
        UiView::Dialogue
    } else {
        UiView::Town
    }
}
