use crate::prelude::*;
use crate::gui::ui_state::UiState;
use crate::gui::input::UiView;


impl UiState {
    pub fn init_leaderboard(&mut self) -> PadlResult<()> {
        let r = self.main_area;
        let pane = panes::new_pane(r.x() as u32, r.y() as u32, r.width() as u32, r.height() as u32, "TEST")?;
        pane.hide()?;
        self.view_panes.push((UiView::Leaderboard, pane));
        Ok(())
    }
}