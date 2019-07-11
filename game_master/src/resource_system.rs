use db_lib::models::*;
use crate::db::DB;

impl DB {
    pub fn collect_reward<'a, I>(&self, units: I) 
    where
        I: IntoIterator<Item = &'a Unit>,
    {
        use std::ops::Add;
        let feathers = units.into_iter().map(reward_feathers).fold(0, i64::add);
        self.add_resource(ResourceType::Feathers, feathers).expect("Adding feathers.");
    }

    pub fn init_resources(&self) {
        use db_lib::strum::IntoEnumIterator;
        for res in ResourceType::iter()
        {
            let entity = Resource {
                resource_type: res, 
                amount: 0,
            };
            match self.insert_resource(&entity) {
                Err(e) => println!("Couldn't insert resource. {} probalbly already exists. Error: {}", res, e),
                _ => {}
            }
        }
    }
}
fn reward_feathers(unit: &Unit) -> i64 {
    let f = (1.0 + unit.hp as f32 * unit.speed / 4.0).log2().ceil() as i64;
    println!("{:#?} gives {} feathers", &unit, f);
    f
}