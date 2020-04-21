use crate::gui::input::*;
use crate::prelude::*;
use quicksilver::prelude::*;
use specs::prelude::*;

// #[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    /// Currently displayed view for easy access.
    /// Duplicate of FrameManager::currentView, should be considered to bre removed here.
    pub current_view: UiView,
}

#[derive(Default)]
/// Global animation ticker
pub struct ClockTick(pub u32);
#[derive(Default)]
/// Real-time timestamp of frame rendering
pub struct Now(pub Timestamp);

impl UiState {
    pub fn new(current_view: UiView) -> Self {
        UiState {
            grabbed_item: None,
            selected_entity: None,
            hovered_entity: None,
            main_area: Rectangle::default(),
            menu_box_area: Rectangle::default(),
            current_view,
        }
    }
    pub fn leave_view(&mut self) {
        self.selected_entity = None;
        self.grabbed_item = None;
    }
}
