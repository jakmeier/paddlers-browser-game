use quicksilver::prelude::*;
use specs::prelude::*;

use super::task_factory::NewTaskDescriptor;
use crate::game::{
    components::*,
    movement::*,
    town::{DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
};
use crate::gui::gui_components::{ClickOutput, Condition, InteractiveTableArea};
use crate::gui::input::{Clickable, Grabbable};
use crate::gui::ui_state::*;
use crate::logging::ErrorQueue;
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;
use crate::prelude::*;
use paddlers_shared_lib::api::shop::Cost;
use paddlers_shared_lib::prelude::*;

impl Town {
    pub fn left_click_on_menu<'a>(
        &mut self,
        menu_entity: Entity,
        mouse_pos: Vector,
        mut ui_state: WriteExpect<'a, UiState>,
        position: ReadStorage<'a, Position>,
        mut workers: WriteStorage<'a, Worker>,
        mut containers: WriteStorage<'a, EntityContainer>,
        ui_menu: &'a mut UiMenu,
        lazy: Read<'a, LazyUpdate>,
        resources: &TownResources,
        mut errq: WriteExpect<'a, ErrorQueue>,
        mut rest: WriteExpect<'a, RestApiState>,
        ep: &EventPool,
    ) {
        let click_output = ui_menu.ui.click(mouse_pos).unwrap_or_else(|e| {
            errq.push(e);
            None
        });
        if let Some((_, Some(condition))) = &click_output {
            let err = check_condition(condition, resources);
            if let Err(e) = err {
                errq.push(e);
                return;
            }
        }
        match click_output {
            Some((ClickOutput::Ability(ability), _)) => {
                (*ui_state).grabbed_item = Some(Grabbable::Ability(ability));
            }
            Some((ClickOutput::Entity(clicked_entity), _)) => {
                if let Some(container) = containers.get_mut(menu_entity) {
                    container.remove_entity(clicked_entity);
                    ui_menu.ui.remove(clicked_entity.into());
                    let container_area = position.get(menu_entity).unwrap().area;
                    let tile = self.tile(container_area.pos);
                    move_worker_out_of_building(
                        self,
                        clicked_entity,
                        container.task,
                        &mut workers,
                        tile,
                        container_area.size(),
                        &lazy,
                        &mut rest,
                    )
                    .unwrap_or_else(|e| errq.push(e));
                } else {
                    errq.push(PadlError::dev_err(PadlErrorCode::DevMsg(
                        "Unexpectedly clicked on an entity",
                    )))
                }
            }
            Some((ClickOutput::Event(e), _)) => {
                // These events COULD be handled here since we have access to most of the game state here
                // However, if an Evnet is modeled as a GameEvent, then the logic to handle it belongs
                // into the game event manger and not here.
                ep.send(e).unwrap_or_else(|_| {
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
    pub fn click_default_shop<'a>(
        mouse_pos: Vector,
        mut ui_state: WriteExpect<'a, UiState>,
        shop: Read<'a, DefaultShop>,
        resources: Write<'a, TownResources>,
    ) -> PadlResult<()> {
        let maybe_grab = shop.click(mouse_pos)?;
        match maybe_grab {
            None => {}
            Some((g, None)) => {
                (*ui_state).grabbed_item = Some(g);
            }
            Some((g, Some(condition))) => {
                check_condition(&condition, &*resources)?;
                (*ui_state).grabbed_item = Some(g);
            }
        }
        Ok(())
    }

    /// Left click on main area
    pub fn left_click<'a>(
        &mut self,
        mouse_pos: Vector,
        entities: &Entities<'a>,
        ui_state: &mut WriteExpect<'a, UiState>,
        position: &ReadStorage<'a, Position>,
        clickable: &ReadStorage<'a, Clickable>,
        net_ids: &ReadStorage<'a, NetObj>,
        lazy: &Read<'a, LazyUpdate>,
        resources: &mut Write<'a, TownResources>,
        errq: &mut WriteExpect<'a, ErrorQueue>,
        rest: &mut WriteExpect<'a, RestApiState>,
    ) -> Option<NewTaskDescriptor> {
        let maybe_top_hit = Self::clickable_lookup(entities, mouse_pos, position, clickable);
        if let Some(grabbed) = &(*ui_state).grabbed_item {
            match grabbed {
                Grabbable::NewBuilding(bt) => {
                    if let Some(pos) = self.get_buildable_tile(mouse_pos) {
                        rest.http_place_building(pos, *bt, current_village())
                            .unwrap_or_else(|e| errq.push(e));
                        resources.spend(&bt.price());
                        self.insert_new_building(&entities, &lazy, pos, *bt);
                        (*ui_state).grabbed_item = None;
                    }
                }
                Grabbable::Ability(a) => {
                    let a = *a;
                    let target = maybe_top_hit.and_then(|e| net_ids.get(e)).map(|n| n.id);
                    (*ui_state).grabbed_item = None;
                    match a {
                        AbilityType::Welcome => {
                            if target.is_some() {
                                return Some((TaskType::WelcomeAbility, target));
                            } else {
                                return None;
                            }
                        }
                        AbilityType::Work => {
                            let job = TaskType::Walk; // TODO: find right job
                            return Some((job, None));
                        }
                    }
                }
            }
        }
        (*ui_state).selected_entity = maybe_top_hit;
        None
    }

    /// Returns the top most entity clickable in the town view
    pub fn clickable_lookup<'a>(
        entities: &Entities<'a>,
        mouse_pos: Vector,
        position: &ReadStorage<'a, Position>,
        clickable: &ReadStorage<'a, Clickable>,
    ) -> Option<Entity> {
        let mut top_hit: Option<(i32, Entity)> = None;
        for (e, pos, _) in (entities, position, clickable).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                if top_hit.is_none() || top_hit.unwrap().0 < pos.z {
                    top_hit = Some((pos.z, e));
                }
            }
        }
        top_hit.map(|tup| tup.1)
    }
}

pub fn check_condition(condition: &Condition, resources: &TownResources) -> PadlResult<()> {
    match condition {
        Condition::HasResources(price) => {
            if resources.can_afford(&price) {
                Ok(())
            } else {
                PadlErrorCode::NotEnoughResources.usr()
            }
        }
    }
}
