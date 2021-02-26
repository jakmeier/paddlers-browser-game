//! TODO: This module structure has not aged well...
//!
//! Right now, this module contains several specs systems, which
//!  1) logically should be where the remaining code of those components is
//!  2) could potentially be rewritten to not use dispatcher at all
use super::{Clickable, MouseState};
use crate::game::{
    components::*,
    movement::*,
    player_info::PlayerState,
    town::nests::Nest,
    town::{tiling, DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
};
use crate::gui::gui_components::{ClickOutput, InteractiveTableArea};
use crate::gui::ui_state::UiState;
use crate::prelude::*;
use specs::prelude::*;

pub struct TownLeftClickSystem;
impl TownLeftClickSystem {
    pub fn new() -> Self {
        TownLeftClickSystem
    }
}

impl<'a> System<'a> for TownLeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        WriteExpect<'a, TownResources>,
        WriteExpect<'a, Town>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
        ReadStorage<'a, Moving>,
        ReadStorage<'a, NetObj>,
        ReadStorage<'a, Mana>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, Worker>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut ui_state,
            mut resources,
            mut town,
            lazy,
            position,
            clickable,
            moving,
            net_ids,
            mana,
            mut containers,
            mut workers,
        ): Self::SystemData,
    ) {
        let active_entity = ui_state.selected_entity;
        let MouseState(mouse_pos) = *mouse_state;

        let maybe_job = town.left_click(
            mouse_pos,
            &entities,
            &mut ui_state,
            &position,
            &clickable,
            &net_ids,
            &lazy,
            &mut resources,
        );
        if let Some(job) = maybe_job {
            let active_entity = active_entity.expect("Ability requires unit");
            let worker = workers
                .get_mut(active_entity)
                .expect("Ability requires unit");
            let (from, movement) = (&position, &moving)
                .join()
                .get(active_entity, &entities)
                .expect("Unit has position");
            let start = tiling::next_tile_in_direction(from.area.pos, movement.momentum);
            let target_tile = tiling::tile(mouse_pos);
            let range = AbilityType::from_task(&job.0)
                .as_ref()
                .map(AbilityType::range)
                .unwrap_or(0.0);
            // TODO: Take movement of visitor into account
            let destination = (*town).closest_walkable_tile_in_range(start, target_tile, range);
            if destination.is_none() {
                nuts::publish(PadlError::user_err(PadlErrorCode::PathBlocked));
                return;
            }
            worker.new_order(
                active_entity,
                start,
                job,
                destination.unwrap(),
                &*town,
                &mut containers,
                &mana,
            );
        }
    }
}

pub struct TownMenuLeftClickSystem;
impl TownMenuLeftClickSystem {
    pub fn new() -> Self {
        TownMenuLeftClickSystem
    }
}

impl<'a> System<'a> for TownMenuLeftClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        ReadExpect<'a, DefaultShop>,
        WriteExpect<'a, TownResources>,
        WriteExpect<'a, Town>,
        ReadExpect<'a, PlayerState>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, UiMenu>,
        WriteStorage<'a, ForeignUiMenu>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, Nest>,
    );

    fn run(
        &mut self,
        (
            mouse_state,
            mut ui_state,
            shop,
            resources,
            mut town,
            player_info,
            lazy,
            position,
            mut containers,
            mut ui_menus,
            mut foreign_ui_menus,
            mut workers,
            mut nests,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos) = *mouse_state;
        let foreign = town.is_foreign();

        if let Some(entity) = (*ui_state).selected_entity {
            let menu = ui_menus.get_mut(entity);
            if let Some(ui_menu) = menu {
                if ui_menu.show(foreign) {
                    town.left_click_on_menu(
                        entity,
                        mouse_pos,
                        &mut ui_state,
                        &position,
                        &mut workers,
                        &mut containers,
                        &mut ui_menu.ui,
                        &mut nests,
                        &lazy,
                        &*resources,
                        player_info.info(),
                    );
                }
            }
            if foreign {
                if let Some(ui_menu) = foreign_ui_menus.get_mut(entity) {
                    town.left_click_on_menu(
                        entity,
                        mouse_pos,
                        &mut ui_state,
                        &position,
                        &mut workers,
                        &mut containers,
                        &mut ui_menu.ui,
                        &mut nests,
                        &lazy,
                        &*resources,
                        player_info.info(),
                    );
                }
            }
        } else {
            Town::click_default_shop(mouse_pos, &mut ui_state, shop, resources, player_info)
                .unwrap_or_else(|e| nuts::publish(e));
        }
    }
}

pub struct MapLeftClickSystem;
impl MapLeftClickSystem {
    pub fn new() -> Self {
        MapLeftClickSystem
    }
}

impl<'a> System<'a> for MapLeftClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        WriteStorage<'a, UiMenu>,
    );

    fn run(&mut self, (mouse_state, ui_state, mut ui_menus): Self::SystemData) {
        let MouseState(mouse_pos) = *mouse_state;
        if let Some(entity) = (*ui_state).selected_entity {
            if let Some(ui_menu) = ui_menus.get_mut(entity) {
                let click_output = ui_menu.ui.click(mouse_pos);
                match click_output {
                    Some((ClickOutput::Event(e), _)) => {
                        crate::game::game_event_manager::game_event(e);
                    }
                    Some((ClickOutput::DoNothing, _)) => { /* Do nothing */ }
                    Some(_) => nuts::publish(PadlError::dev_err(PadlErrorCode::DevMsg(
                        "Unexpectedly clicked something",
                    ))),
                    None => {}
                }
            }
        }
    }
}
