use crate::{db::DB, StringErr};
use paddlers_shared_lib::{api::shop::*, prelude::*};

impl DB {
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
