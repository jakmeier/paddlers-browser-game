pub mod text_pool;
mod frame;
mod text_node;
mod floating_text;
pub use frame::*;
pub use text_node::*;
pub use floating_text::*;

use std::collections::HashMap;
use crate::prelude::*;
use crate::game::Game;
use crate::gui::input::UiView;
use crate::gui::ui_state::UiState;
use specs::{Dispatcher, World};

#[derive(Default)]
pub struct ViewManager<'a,'b> {
    dispatchers: HashMap<UiView, Vec<Dispatcher<'a,'b>>>,
}

// TODO: Remove and replace with what is currently called FrameManager
impl<'a,'b> ViewManager<'a,'b> {
    pub fn add_dispatcher(&mut self, v: UiView, d: Dispatcher<'a,'b>) {
        let entry = self.dispatchers.entry(v).or_insert(vec![]);
        entry.push(d);
    }
    pub fn update(&mut self, world: &mut World, view: UiView) {
        if let Some(view_dispatchers) = self.dispatchers.get_mut(&view) {
            for dispatcher in view_dispatchers.iter_mut() {
                dispatcher.dispatch(world);
            }
        }
    }
}

impl Game<'_,'_> {
    pub fn switch_view(&mut self, view: UiView) -> PadlResult<()> {
        self.leave_view()?;
        self.enter_view(view)?;
        Ok(())
    }
    pub fn toggle_view(&mut self) -> PadlResult<()> {
        let ui: shred::Fetch<UiState> = self.world.fetch();
        let next =
        match (*ui).current_view {
            UiView::Map => UiView::Town,
            UiView::Town => UiView::Attacks,
            UiView::Attacks => UiView::Leaderboard,
            UiView::Leaderboard => UiView::Map,
        };
        std::mem::drop(ui);

        self.switch_view(next)
    }
    fn enter_view(&mut self, view: UiView) -> PadlResult<()> {
        let ui: &mut UiState = &mut *self.world.fetch_mut();
        ui.current_view = view;
        for (v,pane) in ui.view_panes.iter_mut() {
            if *v == ui.current_view {
                pane.show()?;
            } else {
                pane.hide()?;
            }
        }
        Ok(())
    }
    fn leave_view(&mut self) -> PadlResult<()> {
        let ui: &mut UiState = &mut *self.world.fetch_mut();
        ui.leave_view();
        Ok(())
    }    
}