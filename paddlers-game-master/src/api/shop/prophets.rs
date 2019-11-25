use crate::{ActorAddresses,StringErr};
use crate::db::{DB, DeferredDbStatement};
use paddlers_shared_lib::{api::shop::*, prelude::*, game_mechanics::prophets::*};

impl DB {
    fn check_prophet_conditions(&self, p: &Player) -> Result<Price, String> {
        let karma = p.karma;
        let prophets_alive = self.player_prophets_count(p.uuid);
        let villlages_owned = self.player_village_count(p.uuid);

        let total_prophets = prophets_alive + villlages_owned;
        if prophets_allowed(karma) <= total_prophets {
            return Err("Not enough Karma".to_owned());
        }
        Ok(prophet_cost(total_prophets))
    }

    pub fn try_buy_prophet(&self, village: VillageKey, addrs: &ActorAddresses, p: &Player) -> StringErr {
        self.check_prophet_conditions(p)
            .map(|cost| self.try_spend(&cost, village))
            .and_then(|_| {
                addrs
                    .db_actor
                    .try_send(DeferredDbStatement::NewProphet(village))
                    .map_err(|e| format!("{}", e))
            })
    }

    pub fn add_prophet(&self, v: VillageKey) {
        let prophet = NewHobo {
            hp: 10,
            home: v.num(),
            color: Some(UnitColor::Prophet),
            speed: 0.05,
        };
        self.insert_hobo(&prophet);
    }
}
