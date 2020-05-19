use crate::db::{DeferredDbStatement, DB};
use crate::{ActorAddresses, StringErr};
use paddlers_shared_lib::{api::shop::*, game_mechanics::prophets::*, prelude::*};

impl DB {
    fn check_prophet_conditions(&self, p: &Player) -> Result<Price, String> {
        let karma = p.karma;
        let prophets_alive = self.player_prophets_count(p.uuid);
        let villlages_owned = self.player_village_count(p.key());

        let total_prophets = prophets_alive + villlages_owned - 1;
        if prophets_allowed(karma) <= total_prophets {
            return Err("Not enough Karma".to_owned());
        }
        Ok(prophet_cost(total_prophets))
    }

    pub fn try_buy_prophet(
        &self,
        village: VillageKey,
        addrs: &ActorAddresses,
        p: &Player,
    ) -> StringErr {
        self.check_prophet_conditions(p)
            .and_then(|cost| self.try_spend(&cost, village))
            .and_then(|()| {
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
            hurried: true,
            nest: None,
        };
        self.insert_hobo(&prophet);
    }
}
