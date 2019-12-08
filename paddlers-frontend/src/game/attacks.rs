//! View for incoming and outgoing attacks

use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::api::attacks::*;
use crate::prelude::*;
use crate::game::Game;
use crate::net::state::current_village;

impl Game<'_,'_> {
    pub fn send_prophet_attack(&mut self, target: (i32,i32)) -> PadlResult<()> { 
        let prophet = Troop {
            color: UnitColor::Prophet,
            typ: UnitType::Basic,
            count: 1,
        };
        let atk = AttackDescriptor {
            from: current_village(),
            to: target,
            units: vec![prophet],
        };
        self.rest().http_send_attack(atk)?;
        self.town_mut().idle_prophets -= 1;
        Ok(())
    }
}