//! The ability to welcome can be used on visitors.
//! Heroes have this ability from the start, other working units
//! may learn it as well.
//! The effect of the ability is an increased satisfaction,

use quicksilver::prelude::Vector;
use specs::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::api::abilities::AbilityUse;
use crate::game::components::*;
use crate::net::game_master_api::RestApiState;
use crate::logging::ErrorQueue;
use crate::game::town::Town;

pub fn use_welcome_ability<'a>(
    net_id: &NetObj,
    mouse_pos: Vector,
    rest: &mut RestApiState,
    errq: &mut ErrorQueue,
    position: &ReadStorage<'a, Position>,
    clickable: &ReadStorage<'a, Clickable>,
    health: &mut WriteStorage<'a, Health>,
    entities: &Entities<'a>,
) {
    // XXX For now, no checks, just increase happiness
    if let Some(visitor) = Town::clickable_lookup(entities, mouse_pos, position, clickable) {
        if let Some(h) = health.get_mut(visitor) {
            h.make_happy(1);
        }
    }
    rest.http_use_ability( AbilityUse {
        unit_id: UnitKey(net_id.id),
        ability_type: AbilityType::Welcome,
    })
    .unwrap_or_else(|e| errq.push(e));
}