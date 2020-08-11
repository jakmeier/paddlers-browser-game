mod floating_text;
mod text_node;
pub mod text_pool;
use crate::game::story::select_dialogue_scene;
pub use floating_text::*;
use paddlers_shared_lib::story::story_state::StoryState;
pub use text_node::*;

use crate::gui::ui_state::UiState;
use crate::prelude::*;

impl Game<'_, '_> {
    pub fn switch_view(&mut self, view: UiView) {
        {
            let ui: &mut UiState = &mut *self.world.fetch_mut();
            ui.leave_view();
        }
        self.world.insert(view);
    }
    pub fn toggle_view(&mut self) {
        let view = *self.world.fetch::<UiView>();
        let next = match view {
            UiView::Map => UiView::Town,
            UiView::Town => UiView::Visitors(VisitorViewTab::Letters),
            UiView::Visitors(_) => UiView::Leaderboard,
            UiView::Leaderboard => UiView::Map,
            UiView::Dialogue => return,
        };

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
