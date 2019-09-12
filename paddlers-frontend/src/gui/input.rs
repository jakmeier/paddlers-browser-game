/// This module keeps the logic to read input and, in most cases,
/// redirect it to suitable modules to handle the input

use crate::net::game_master_api::RestApiState;
use quicksilver::geom::{Vector, Shape, Rectangle};
use quicksilver::prelude::MouseButton;
use specs::prelude::*;
use crate::gui::{
    menu::buttons::MenuButtons,
};
use crate::game::{
    movement::*,
    town_resources::TownResources,
    town::{Town, town_shop::DefaultShop},
    units::workers::*,
    components::*,
};
use crate::logging::ErrorQueue;
use paddlers_shared_lib::prelude::*;

pub mod pointer;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub Option<MouseButton>);

#[derive(Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub menu_box_area: Rectangle,
    pub current_view: UiView,
}
#[derive(Clone, Copy, Debug)]
pub enum UiView {
    Town,
    Map,
}

pub struct LeftClickSystem;
pub struct RightClickSystem;
pub struct HoverSystem;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone)]
pub enum Grabbable {
    NewBuilding(BuildingType),
}

impl<'a> System<'a> for LeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, DefaultShop>,
        ReadExpect<'a, MenuButtons>,
        Write<'a, TownResources>,
        Write<'a, Town>,
        WriteExpect<'a, RestApiState>,
        WriteExpect<'a, ErrorQueue>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, Worker>,
     );

    fn run(&mut self, 
        (
            entities, 
            mouse_state, 
            mut ui_state, 
            shop, 
            buttons, 
            resources, 
            mut town, 
            rest, 
            errq, 
            lazy, 
            position, 
            clickable, 
            containers, 
            workers
        ): Self::SystemData) 
    {

        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Left) {
            return;
        }
        
        // Always visible buttons 
        buttons.click(mouse_pos, &mut *ui_state);

        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        
        match (ui_state.current_view, in_menu_area) {
            (UiView::Town, true) => {
                town.left_click_on_menu(mouse_pos, ui_state, position, workers, containers, lazy, shop, resources, errq, rest);
            },
            (UiView::Map, true) => {
                // NOP
            }
            (UiView::Town, false) => {
                town.left_click(mouse_pos, entities, ui_state, position, clickable, lazy, resources, errq, rest);
            },
            (UiView::Map, false) => {
                // NOP
            },
        }
    }
}

impl<'a> System<'a> for RightClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, Town>,
        WriteExpect<'a, RestApiState>,
        WriteExpect<'a, ErrorQueue>,
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Moving>,
        ReadStorage<'a, EntityContainer>,
     );

    fn run(&mut self, (mouse_state, mut ui_state, town, mut rest, mut errq, entities, mut worker, position, moving, containers): Self::SystemData) {

        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Right) {
            return;
        }

        (*ui_state).grabbed_item = None;


        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        
        match (ui_state.current_view, in_menu_area) {
            (_, true) => {
                // NOP
            },
            (UiView::Map, false) => {
                // NOP
            },
            (UiView::Town, false) => {
                if let Some((worker, from, movement)) = 
                    ui_state.selected_entity
                    .and_then(
                        |selected| 
                        (&mut worker, &position, &moving).join().get(selected, &entities) 
                    )
                {
                    let start = town.next_tile_in_direction(from.area.pos, movement.momentum);                
                    let msg = worker.task_on_right_click(start, &mouse_pos, &town, &containers);
                    match msg {
                        Ok(Some(msg)) => {
                            rest.http_overwrite_tasks(msg)
                                .unwrap_or_else(|e| errq.push(e));
                        }
                        Ok(None) => { },
                        Err(e) => {
                            errq.push(e);
                        }
                    }
                }
            },
        }
    }
}

impl<'a> System<'a> for HoverSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        ReadStorage<'a, Position>,
     );

    fn run(&mut self, (entities, mouse_state, mut ui_state, position): Self::SystemData) {

        let MouseState(mouse_pos, _) = *mouse_state;
        
        (*ui_state).hovered_entity = None;
        
        match (*ui_state).current_view {
            UiView::Map => {},
            UiView::Town => {
                for (e, pos) in (&entities, &position).join() {
                    if mouse_pos.overlaps_rectangle(&pos.area) {
                        (*ui_state).hovered_entity = Some(e);
                        break;
                    }
                }
            }
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            grabbed_item: None,
            selected_entity: None,
            hovered_entity: None,
            menu_box_area: Rectangle::default(),
            current_view: UiView::Town,
        }
    }
}
impl UiState {
    pub fn toggle_view(&mut self) {
        match self.current_view {
            UiView::Map => self.current_view = UiView::Town,
            UiView::Town => self.current_view = UiView::Map,
        }
    }
    pub fn set_view(&mut self, view: UiView) {
        self.current_view = view;
    }
}