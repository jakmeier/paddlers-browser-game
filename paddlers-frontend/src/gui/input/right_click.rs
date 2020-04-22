use super::{MouseState, UiView};
use crate::game::{components::*, movement::*, town::Town, units::workers::*};
use crate::gui::ui_state::UiState;
use crate::logging::ErrorQueue;
use crate::net::game_master_api::RestApiState;
use quicksilver::prelude::*;
use specs::prelude::*;

/// TODO: Remove RightClickSystem
pub struct RightClickSystem;

impl<'a> System<'a> for RightClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        Read<'a, Town>,
        WriteExpect<'a, RestApiState>,
        WriteExpect<'a, ErrorQueue>,
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Moving>,
        ReadStorage<'a, Clickable>,
        ReadStorage<'a, NetObj>,
        ReadStorage<'a, Mana>,
        WriteStorage<'a, EntityContainer>,
    );

    fn run(
        &mut self,
        (
            mouse_state,
            mut ui_state,
            town,
            mut rest,
            mut errq,
            entities,
            mut worker,
            position,
            moving,
            clickable,
            net_ids,
            mana,
            mut containers,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Right) {
            return;
        }

        if (*ui_state).grabbed_item.take().is_some() {
            return;
        }

        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        let maybe_top_hit = Town::clickable_lookup(&entities, mouse_pos, &position, &clickable);

        match (ui_state.current_view, in_menu_area) {
            (_, true) => {
                // NOP
            }
            (UiView::Map, false) => {
                // NOP
            }
            (UiView::Town, false) => {
                if let Some(e) = (*ui_state).selected_entity {
                    if let Some(worker) = worker.get_mut(e) {
                        let maybe_job = worker.task_on_right_click(&mouse_pos, &town);
                        if let Some((job, destination)) = maybe_job {
                            let target = maybe_top_hit.and_then(|e| net_ids.get(e)).map(|n| n.id);
                            let (from, movement) =
                                (&position, &moving).join().get(e, &entities).unwrap();
                            let start =
                                town.next_tile_in_direction(from.area.pos, movement.momentum);
                            let new_job = (job, target);
                            worker.new_order(
                                e,
                                start,
                                new_job,
                                destination,
                                &*town,
                                &mut *rest,
                                &mut *errq,
                                &mut containers,
                                &mana,
                            );
                        }
                    }
                }
            }
            (UiView::Attacks, false) => {
                // NOP
            }
            (UiView::Leaderboard, false) => {
                // NOP
            }
            (UiView::Dialogue, false) => {
                // NOP
            }
        }
    }
}
