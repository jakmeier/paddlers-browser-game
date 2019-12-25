use specs::prelude::*;
use quicksilver::prelude::*;
use crate::prelude::*;
use crate::game::Game;
use crate::gui::input::*;

// #[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    pub current_view: UiView,
    pub view_panes: Vec<(UiView, panes::PaneHandle)>,
}


#[derive(Default)]
/// Global animation ticker
pub struct ClockTick(pub u32);
#[derive(Default)]
/// Used for UI scaling. To be removed in favour of better options.
pub struct UnitLength(pub f32);
#[derive(Default)]
/// Real-time timestamp of frame rendering
pub struct Now(pub Timestamp);

impl Default for UiState {
    fn default() -> Self {
        UiState {
            grabbed_item: None,
            selected_entity: None,
            hovered_entity: None,
            main_area: Rectangle::default(),
            menu_box_area: Rectangle::default(),
            current_view: UiView::Town,
            view_panes: vec![],
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
            UiView::Attacks => UiView::Map,
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
        ui.selected_entity = None;
        ui.grabbed_item = None;
        Ok(())
    }    
}