use crate::gui::input::*;
use chrono::NaiveDateTime;
use specs::prelude::*;

#[derive(Clone)]
/// UI state that goes beyond frames
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    grabbed_item: Option<Grabbable>,
}

#[derive(Default, Copy, Clone)]
/// Global animation ticker
pub struct ClockTick(pub u32);
#[derive(Copy, Clone)]
/// Real-time timestamp of frame rendering
pub struct Now(pub NaiveDateTime);

impl Default for Now {
    fn default() -> Self {
        Self(NaiveDateTime::from_timestamp(0, 0))
    }
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            grabbed_item: None,
            selected_entity: None,
            hovered_entity: None,
        }
    }
    pub fn leave_view(&mut self) {
        self.selected_entity = None;
        self.grabbed_item = None;
    }
    #[inline]
    pub fn take_grabbed_item(&mut self) -> Option<Grabbable> {
        self.grabbed_item.take()
    }
    #[inline]
    pub fn grabbed_item(&self) -> &Option<Grabbable> {
        &self.grabbed_item
    }
    #[inline]
    pub fn set_grabbed_item(&mut self, g: Grabbable) {
        self.grabbed_item = Some(g)
    }
}
