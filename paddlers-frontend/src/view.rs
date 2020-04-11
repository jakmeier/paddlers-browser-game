mod floating_text;
mod frame;
mod text_node;
pub mod text_pool;
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
            UiView::Dialog => return,
        };
        std::mem::drop(ui);

        self.switch_view(next);
    }
}

impl UiView {
    /// Determines in which view the game is loaded.
    /// Requieres the player's current story state.
    pub fn entry(story_state: &StoryState) -> Self {
        match story_state {
            StoryState::Initialized
            | StoryState::TempleBuilt
            | StoryState::VisitorArrived
            | StoryState::FirstVisitorWelcomed
            | StoryState::FlowerPlanted
            | StoryState::MoreHappyVisitors
            | StoryState::TreePlanted
            | StoryState::StickGatheringStationBuild
            | StoryState::GatheringSticks => UiView::Town,
            StoryState::ServantAccepted => UiView::Dialog,
        }
    }
}
