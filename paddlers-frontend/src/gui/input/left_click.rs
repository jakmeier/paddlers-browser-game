use super::{Clickable, MouseState};
use crate::game::{
    components::*,
    movement::*,
    town::{DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
};
use crate::gui::gui_components::{ClickOutput, InteractiveTableArea};
use crate::gui::ui_state::UiState;
use crate::logging::ErrorQueue;
use crate::prelude::*;
use specs::prelude::*;

pub struct TownLeftClickSystem {
    _event_pool: EventPool,
}
impl TownLeftClickSystem {
    pub fn new(_event_pool: EventPool) -> Self {
        TownLeftClickSystem { _event_pool }
    }
}

impl<'a> System<'a> for TownLeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        WriteExpect<'a, TownResources>,
        WriteExpect<'a, Town>,
        WriteExpect<'a, ErrorQueue>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
        ReadStorage<'a, Moving>,
        ReadStorage<'a, NetObj>,
        ReadStorage<'a, Mana>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, Worker>,
        // TODO: Only temporary experiment
        WriteExpect<'a, crate::view::ExperimentalSignalChannel>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut ui_state,
            mut resources,
            mut town,
            mut errq,
            lazy,
            position,
            clickable,
            moving,
            net_ids,
            mana,
            mut containers,
            mut workers,
            mut signals,
        ): Self::SystemData,
    ) {
        let active_entity = ui_state.selected_entity;
        let MouseState(mouse_pos, _button) = *mouse_state;

        let maybe_job = town.left_click(
            mouse_pos,
            &entities,
            &mut ui_state,
            &position,
            &clickable,
            &net_ids,
            &lazy,
            &mut resources,
            &mut errq,
            &mut signals,
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
            let start = town
                .resolution
                .next_tile_in_direction(from.area.pos, movement.momentum);
            let target_tile = town.resolution.tile(mouse_pos);
            let range = AbilityType::from_task(&job.0)
                .as_ref()
                .map(AbilityType::range)
                .unwrap_or(0.0);
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
                &mut *errq,
                &mut containers,
                &mana,
            );
        }
    }
}

pub struct TownMenuLeftClickSystem {
    event_pool: EventPool,
}
impl TownMenuLeftClickSystem {
    pub fn new(event_pool: EventPool) -> Self {
        TownMenuLeftClickSystem { event_pool }
    }
}

impl<'a> System<'a> for TownMenuLeftClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        ReadExpect<'a, DefaultShop>,
        WriteExpect<'a, TownResources>,
        WriteExpect<'a, Town>,
        WriteExpect<'a, ErrorQueue>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, UiMenu>,
        WriteStorage<'a, Worker>,
    );

    fn run(
        &mut self,
        (
            mouse_state,
            ui_state,
            shop,
            resources,
            mut town,
            mut errq,
            lazy,
            position,
            containers,
            mut ui_menus,
            workers,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos, _button) = *mouse_state;

        if let Some(entity) = (*ui_state).selected_entity {
            if let Some(ui_menu) = ui_menus.get_mut(entity) {
                town.left_click_on_menu(
                    entity,
                    mouse_pos,
                    ui_state,
                    position,
                    workers,
                    containers,
                    ui_menu,
                    lazy,
                    &*resources,
                    errq,
                    &self.event_pool,
                );
            }
        } else {
            Town::click_default_shop(mouse_pos, ui_state, shop, resources)
                .unwrap_or_else(|e| errq.push(e));
        }
    }
}

pub struct MapLeftClickSystem {
    event_pool: EventPool,
}
impl MapLeftClickSystem {
    pub fn new(event_pool: EventPool) -> Self {
        MapLeftClickSystem { event_pool }
    }
}

impl<'a> System<'a> for MapLeftClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        WriteExpect<'a, ErrorQueue>,
        WriteStorage<'a, UiMenu>,
    );

    fn run(&mut self, (mouse_state, ui_state, mut errq, mut ui_menus): Self::SystemData) {
        let MouseState(mouse_pos, _button) = *mouse_state;
        if let Some(entity) = (*ui_state).selected_entity {
            if let Some(ui_menu) = ui_menus.get_mut(entity) {
                let click_output = ui_menu.ui.click(mouse_pos).unwrap_or_else(|e| {
                    errq.push(e);
                    None
                });
                match click_output {
                    Some((ClickOutput::Event(e), _)) => {
                        self.event_pool.send(e).unwrap_or_else(|_| {
                            errq.push(PadlError::dev_err(PadlErrorCode::DevMsg(
                                "MPSC send failure",
                            )))
                        });
                    }
                    Some(_) => errq.push(PadlError::dev_err(PadlErrorCode::DevMsg(
                        "Unexpectedly clicked something",
                    ))),
                    None => {}
                }
            }
        }
    }
}
