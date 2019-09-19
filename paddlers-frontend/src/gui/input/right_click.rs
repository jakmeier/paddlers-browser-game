use quicksilver::prelude::*;
use specs::prelude::*;
use crate::net::game_master_api::RestApiState;
use crate::game::{
    movement::*,
    town::Town,
    units::workers::*,
    components::*,
};
use crate::logging::ErrorQueue;
use super::{UiState, UiView, MouseState};

pub struct RightClickSystem;

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
        WriteStorage<'a, EntityContainer>,
     );

    fn run(&mut self, (mouse_state, mut ui_state, town, mut rest, mut errq, entities, mut worker, position, moving, containers): Self::SystemData) {

        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Right) {
            return;
        }

        if (*ui_state).grabbed_item.take().is_some() {
            return;
        }


        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        
        match (ui_state.current_view, in_menu_area) {
            (_, true) => {
                // NOP
            },
            (UiView::Map, false) => {
                // NOP
            },
            (UiView::Town, false) => {
                if let Some(e) = (*ui_state).selected_entity {
                    if let Some(worker) = worker.get_mut(e) {
                        worker.new_order(e, &*town, mouse_pos, &mut *rest, &mut *errq, &position, &moving, &containers, &entities);
                    }
                }
            },
        }
    }
}

