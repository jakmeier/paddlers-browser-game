use crate::db::DB;
use paddlers_shared_lib::{
    prelude::*,
    api::{
        shop::*,
    },
};
use crate::StringErr;

impl DB {
    fn check_prophet_conditions(&self,  village: VillageKey) -> Result<Price,String> {
        // TODO: Prophet conditions check
        

        Ok(prophet_cost())
    }

    pub fn try_buy_prophet(&self, village: VillageKey) -> StringErr {
        self.check_prophet_conditions(village)
            .map(
                |cost| self.try_spend(&cost, village)
            ).map(
                |_| {
                    self.add_prophet(village);
                }
            )
    }

    fn add_prophet(&self, v: VillageKey) {
        let prophet = NewHobo {
            hp: 10,
            home: v.num(),
            color: Some(UnitColor::Prophet),
            speed: 0.05,
        };
        self.insert_hobo(&prophet);
    }
}
