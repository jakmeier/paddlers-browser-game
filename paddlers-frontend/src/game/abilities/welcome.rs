//! The ability to welcome can be used on visitors.
//! Heroes have this ability from the start, other working units
//! may learn it as well.
//! The effect of the ability is an increased satisfaction,

use crate::game::components::*;
use crate::logging::error::*;
use paddlers_shared_lib::models::AbilityType;
use specs::prelude::*;

pub fn use_welcome_ability<'a>(
    user: Entity,
    target: Entity,
    health: &mut WriteStorage<'a, Health>,
    status_effects: &mut WriteStorage<StatusEffects>,
    mana: &mut WriteStorage<Mana>,
) -> PadlResult<()> {
    let h = health
        .get_mut(target)
        .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent(
            "Health for hobo",
        )))?;
    let se = status_effects.get_mut(target).ok_or(PadlError::dev_err(
        PadlErrorCode::MissingComponent("Status effects for hobo"),
    ))?;
    let m = mana
        .get_mut(user)
        .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent(
            "Mana for worker",
        )))?;

    let a = AbilityType::Welcome;
    let strength = a.apply().1;
    let mana_cost = a.mana_cost();

    h.make_happy(strength as i64, target);
    se.add_health_reduction(strength);
    m.mana -= mana_cost;

    Ok(())
}
