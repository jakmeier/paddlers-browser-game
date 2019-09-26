//! The ability to welcome can be used on visitors.
//! Heroes have this ability from the start, other working units
//! may learn it as well.
//! The effect of the ability is an increased satisfaction,

use specs::prelude::*;
use crate::logging::error::*;
use crate::game::components::*;
use paddlers_shared_lib::models::AbilityType;

pub fn use_welcome_ability<'a>(
    target: Entity,
    health: &mut WriteStorage<'a, Health>,
    status_effects: &mut WriteStorage<StatusEffects>,
) -> PadlResult<()>
{
    let h = health.get_mut(target)
        .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent("Health for entity")))?;
    let se = status_effects.get_mut(target)
        .ok_or(PadlError::dev_err(PadlErrorCode::MissingComponent("Status effects for entity")))?;

    let strength = AbilityType::Welcome.apply().1;
    h.make_happy(strength as i64);
    se.add_health_reduction(strength);
    Ok(())
}