pub mod text_pool;
mod frame;
mod text_node;
mod floating_text;
pub use frame::*;
pub use text_node::*;
pub use floating_text::*;

use crate::game::Game;
use crate::gui::input::UiView;
use crate::gui::ui_state::UiState;

impl Game<'_,'_> {
    pub fn switch_view(&mut self, view: UiView) {
        let ui: &mut UiState = &mut *self.world.fetch_mut();
        ui.leave_view();
        ui.current_view = view;
    }
    pub fn toggle_view(&mut self) {
        let ui: shred::Fetch<UiState> = self.world.fetch();
        let next =
        match (*ui).current_view {
            UiView::Map => UiView::Town,
            UiView::Town => UiView::Attacks,
            UiView::Attacks => UiView::Leaderboard,
            UiView::Leaderboard => UiView::Map,
        };
        std::mem::drop(ui);

        self.switch_view(next);
    }
}