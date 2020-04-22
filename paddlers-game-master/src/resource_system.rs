use crate::{db::DB, StringErr};
use paddlers_shared_lib::{api::shop::*, prelude::*};

impl DB {
    pub fn collect_reward<'a, I>(&self, units: I, village: VillageKey, player: Option<PlayerKey>)
    where
        I: IntoIterator<Item = &'a Hobo> + Clone,
    {
        use std::ops::Add;
        let feathers = units
            .clone()
            .into_iter()
            .map(reward_feathers)
            .fold(0, i64::add);
        self.add_resource(ResourceType::Feathers, village, feathers)
            .expect("Adding feathers.");
        if let Some(player) = player {
            let karma = units.into_iter().map(reward_karma).fold(0, i64::add);
            self.add_karma(player, karma).expect("Adding karma.");
        }
    }

    pub fn init_resources(&self, vid: VillageKey) {
        use paddlers_shared_lib::strum::IntoEnumIterator;
        for res in ResourceType::iter() {
            let entity = Resource {
                resource_type: res,
                amount: 0,
                village_id: vid.num(),
            };
            if self.maybe_resource(res, vid).is_none() {
                match self.insert_resource(&entity) {
                    Err(e) => println!("Couldn't insert resource. {} Error: {}", res, e),
                    _ => {}
                }
            }
        }
    }

    pub fn try_spend(&self, p: &Price, village: VillageKey) -> StringErr {
        self.can_afford(p, village)?;
        self.spend(p, village);
        Ok(())
    }

    pub fn spend(&self, p: &Price, village: VillageKey) {
        for (res, n) in p.0.iter() {
            self.add_resource((*res).into(), village, -*n)
                .expect("Unchecked spending resources");
        }
    }
    pub fn can_afford(&self, p: &Price, village: VillageKey) -> StringErr {
        for (res, n) in p.0.iter() {
            if self.resource((*res).into(), village) < *n {
                return Err(format!("Not enough {}", res));
            }
        }
        Ok(())
    }
}
fn reward_feathers(unit: &Hobo) -> i64 {
    let f = (1.0 + unit.hp as f32 * unit.speed / 4.0).log2().ceil() as i64;
    // println!("{:#?} gives {} feathers", &unit, f);
    f
}
fn reward_karma(_unit: &Hobo) -> i64 {
    1
}
