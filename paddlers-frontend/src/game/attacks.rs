//! View for incoming and outgoing attacks

use paddlers_shared_lib::api::attacks::*;
use crate::prelude::*;
use crate::game::Game;
use crate::net::state::current_village;

impl Game<'_,'_> {
    pub fn send_prophet_attack(&mut self, target: (i32,i32)) -> PadlResult<()> {
        let maybe_prophet = self.town_mut().idle_prophets.pop();
        if let Some(prophet) = maybe_prophet {
            let hobo = self.hobo_key(prophet)?;
            let atk = AttackDescriptor {
                from: current_village(),
                to: target,
                units: vec![hobo],
            };
            self.rest().http_send_attack(atk)?;
            Ok(())
        } else {
            PadlErrorCode::NotEnoughUnits.usr()
        }
    }
}