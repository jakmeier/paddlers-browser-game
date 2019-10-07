use super::{Clickable, MouseState, UiState, UiView};
use crate::game::{
    components::*,
    map::{GlobalMapSharedState, MapPosition},
    movement::*,
    town::{town_shop::DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
};
use crate::gui::menu::buttons::MenuButtons;
use crate::logging::ErrorQueue;
use crate::net::game_master_api::RestApiState;
use quicksilver::prelude::*;
use specs::prelude::*;
use crate::prelude::*;

pub struct LeftClickSystem;

impl<'a> System<'a> for LeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, DefaultShop>,
        ReadExpect<'a, MenuButtons>,
        Write<'a, TownResources>,
        Write<'a, Town>,
        Write<'a, GlobalMapSharedState>,
        WriteExpect<'a, RestApiState>,
        WriteExpect<'a, ErrorQueue>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, MapPosition>,
        ReadStorage<'a, Clickable>,
        ReadStorage<'a, Moving>,
        ReadStorage<'a, NetObj>,
        ReadStorage<'a, Mana>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, UiMenu>,
        WriteStorage<'a, Worker>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut ui_state,
            shop,
            buttons,
            mut resources,
            mut town,
            mut map,
            mut rest,
            mut errq,
            lazy,
            position,
            map_position,
            clickable,
            moving,
            net_ids,
            mana,
            mut containers,
            mut ui_menus,
            mut workers,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Left) {
            return;
        }

        let active_entity = ui_state.selected_entity;

        // Always visible buttons
        buttons.click(mouse_pos, &mut *ui_state);

        // Demultiplex signal to views
        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        match (ui_state.current_view, in_menu_area) {
            (UiView::Town(_), true) => {
                if let Some(entity) = (*ui_state).selected_entity {
                    if let Some(ui_menu) = ui_menus.get_mut(entity) {
                        town.left_click_on_menu(
                            entity, mouse_pos, ui_state, position, workers, containers, ui_menu, lazy, errq, rest,
                        );
                    }
                }
                else {
                    Town::click_default_shop(mouse_pos, ui_state, shop, resources);
                }
            }
            (UiView::Map, true) => {
                // NOP
            }
            (UiView::Town(_), false) => {
                let maybe_job =
                    town.left_click(
                        mouse_pos, &entities, &mut ui_state, &position, &clickable, &net_ids, &lazy, &mut resources, &mut errq, &mut rest,
                    );
                if let Some(job) = maybe_job {
                    let active_entity = active_entity.expect("Ability requires unit");
                    let worker = workers.get_mut(active_entity).expect("Ability requires unit");
                    let (from, movement) = (&position, &moving).join().get(active_entity, &entities).expect("Unit has position");
                    let start = town.next_tile_in_direction(from.area.pos, movement.momentum);
                    let target_tile = town.tile(mouse_pos);
                    let range = AbilityType::from_task(&job.0).as_ref().map(AbilityType::range).unwrap_or(0.0);
                    // TODO: Take movement of visitor into account
                    let destination = (*town).closest_walkable_tile_in_range(start, target_tile, range);
                    if destination.is_none() {
                        errq.push(PadlError::user_err(PadlErrorCode::PathBlocked));
                        return;
                    }
                    worker.new_order(
                        active_entity,
                        start,
                        job,
                        destination.unwrap(),
                        &*town,
                        &mut *rest,
                        &mut *errq,
                        &mut containers,
                        &mana,
                    );
                }
            }
            (UiView::Map, false) => map.left_click_on_main_area(mouse_pos, ui_state, entities, map_position, clickable),
        }
    }
}
