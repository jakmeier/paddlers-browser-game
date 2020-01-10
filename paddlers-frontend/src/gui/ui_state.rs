use specs::prelude::*;
use quicksilver::prelude::*;
use crate::prelude::*;
use crate::gui::input::*;

// #[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    pub current_view: UiView,
}


#[derive(Default)]
/// Global animation ticker
pub struct ClockTick(pub u32);
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
        }
    }
}


impl UiState {
    pub fn leave_view(&mut self) {
        self.selected_entity = None;
        self.grabbed_item = None;
    }    
}