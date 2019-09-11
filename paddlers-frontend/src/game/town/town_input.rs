use specs::prelude::*;
use quicksilver::prelude::*;

use crate::net::game_master_api::RestApiState;
use crate::logging::ErrorQueue;
use crate::game::{
    movement::*,
    town_resources::TownResources,
    units::workers::*,
    components::*,
    town::{Town, town_shop::DefaultShop},
};
use crate::gui::input::{Grabbable, UiState, Clickable};
use paddlers_shared_lib::api::shop::Cost;


impl Town {

    pub fn left_click_on_menu<'a> (
        &mut self,
        mouse_pos: Vector, 
        mut ui_state:  Write<'a, UiState>, 
        position: ReadStorage<'a, Position>, 
        mut workers: WriteStorage<'a, Worker>,
        mut containers: WriteStorage<'a, EntityContainer>,
        lazy: Read<'a, LazyUpdate>,
        shop: Read<'a, DefaultShop>,
        resources: Write<'a, TownResources>,
        mut errq: WriteExpect<'a, ErrorQueue>,
        mut rest: WriteExpect<'a, RestApiState>,
        ) 
    {
        if let Some(entity) = (*ui_state).selected_entity {
            if let Some(container) = containers.get_mut(entity) {
                let container_area = position.get(entity).unwrap().area;
                let worker_e = container.worker_to_release(&mouse_pos);
                let tile = self.tile(container_area.pos);
                if let Some(worker_e) = worker_e {
                    move_worker_out_of_building(
                        self,
                        worker_e,
                        container.task,
                        &mut workers,
                        tile, 
                        container_area.size(),
                        &lazy,
                        &mut rest,
                    ).unwrap_or_else(
                        |e|
                        errq.push(e)
                    );
                }
            }
        }
        else {
            let maybe_grab = shop.click(mouse_pos);
            if let Some(Grabbable::NewBuilding(b)) = maybe_grab {
                if (*resources).can_afford(&b.price()) {
                    (*ui_state).grabbed_item = maybe_grab;
                }
            }
        }
    }

    pub fn left_click<'a>(
        &mut self,
        mouse_pos: Vector,
        entities: Entities<'a>,
        mut ui_state:  Write<'a, UiState>, 
        position: ReadStorage<'a, Position>, 
        clickable: ReadStorage<'a, Clickable>,
        lazy: Read<'a, LazyUpdate>,
        mut resources: Write<'a, TownResources>,
        mut errq: WriteExpect<'a, ErrorQueue>,
        mut rest: WriteExpect<'a, RestApiState>,
    ) {
        (*ui_state).selected_entity = None;
        let mut top_hit: Option<(i32, Entity)> = None;
        for (e, pos, _) in (&entities, &position, &clickable).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                if  top_hit.is_none() 
                ||  top_hit.unwrap().0 < pos.z {
                    top_hit = Some((pos.z,e));
                }
            }
        }
        (*ui_state).selected_entity = top_hit.map(|tup| tup.1);
        if let Some(grabbed) = &(*ui_state).grabbed_item {
            match grabbed {
                Grabbable::NewBuilding(bt) => {
                    if let Some(pos) = self.get_buildable_tile(mouse_pos) {
                        rest.http_place_building(pos, *bt).unwrap_or_else(
                            |e|
                            errq.push(e)
                        );
                        resources.spend(&bt.price());
                        self.insert_new_bulding(&entities, &lazy, pos, *bt);
                        (*ui_state).grabbed_item = None;
                    }
                },
            }
        }
    }
}