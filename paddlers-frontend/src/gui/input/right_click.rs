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

