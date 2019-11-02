/// This module keeps the logic to read input and, in most cases,
/// redirect it to suitable modules to handle the input

use quicksilver::geom::{Vector, Rectangle};
use quicksilver::prelude::MouseButton;
use specs::prelude::*;
use paddlers_shared_lib::prelude::*;

pub mod pointer;
pub mod drag;
pub mod left_click;
pub mod right_click;
pub mod hover;

pub use self::{
    left_click::*,
    right_click::*,
    hover::*,
};

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub Option<MouseButton>);

#[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub main_area: Rectangle,
    pub menu_box_area: Rectangle,
    pub current_view: UiView,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiView {
    Town,
    Map,
}

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone)]
pub enum Grabbable {
    NewBuilding(BuildingType),
    Ability(AbilityType),
}

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
    pub fn toggle_view(&mut self) {
        self.reset_view();
        match self.current_view {
            UiView::Map => self.current_view = UiView::Town,
            UiView::Town => self.current_view = UiView::Map,
        }
    }
    pub fn set_view(&mut self, view: UiView) {
        self.reset_view();
        self.current_view = view;
    }
    fn reset_view(&mut self) {
        self.selected_entity = None;
        self.grabbed_item = None;
    }
}