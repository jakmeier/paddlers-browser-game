use crate::gui::input::*;
use chrono::NaiveDateTime;
use paddle::quicksilver_compat::*;
use specs::prelude::*;

#[derive(Clone)]
/// UI state that goes beyond frames
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    grabbed_item: Option<Grabbable>,
}

#[derive(Clone)]
/// State for current view, which probably should be removed eventually because it is redundant
pub struct ViewState {
    // TODO [0.1.4]: I think these four could go into frames.
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    pub inner_menu_box_area: Rectangle,
    pub button_area: Rectangle,
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
impl ViewState {
    pub fn new() -> Self {
        Self {
            main_area: Rectangle::default(),
            menu_box_area: Rectangle::default(),
            inner_menu_box_area: Rectangle::default(),
            button_area: Rectangle::default(),
        }
    }
}
