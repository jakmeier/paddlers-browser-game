use crate::game::story::select_dialogue_scene;
use paddlers_shared_lib::story::story_state::StoryState;

use crate::gui::ui_state::UiState;
use crate::prelude::*;

impl Game {
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
            UiView::Town | UiView::TownHelp => UiView::Mailbox,
            UiView::Mailbox => UiView::Leaderboard(LeaderboardViewTab::KarmaLeaderboard),
            UiView::Leaderboard(LeaderboardViewTab::KarmaLeaderboard) => {
                UiView::Leaderboard(LeaderboardViewTab::IncomingAttacks)
            }
            UiView::Leaderboard(LeaderboardViewTab::IncomingAttacks) => UiView::Map,
            UiView::Dialogue | UiView::Religion => return,
            UiView::Quests => UiView::Town,
        };

        self.switch_view(next);
    }
    pub fn toggle_help_view(&mut self) {
        let view = *self.world.fetch::<UiView>();
        let next = match view {
            UiView::Town => UiView::TownHelp,
            UiView::TownHelp => UiView::Town,
            _ => view,
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
