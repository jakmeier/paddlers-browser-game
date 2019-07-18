use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::shop::*;
use crate::net::graphql::resources_query;

#[derive(Default,Debug, Clone, Copy)]
pub struct TownResources {
    feathers: i64,
    sticks: i64,
    logs: i64,
}

impl TownResources { 
    pub fn read(&self, rt: ResourceType) -> i64 {
        match rt {
            ResourceType::Feathers => self.feathers,
            ResourceType::Sticks => self.sticks,
            ResourceType::Logs=> self.logs,
        }
    }
    fn write(&mut self, rt: ResourceType) -> &mut i64 {
        match rt {
            ResourceType::Feathers => &mut self.feathers,
            ResourceType::Sticks => &mut self.sticks,
            ResourceType::Logs=> &mut self.logs,
        }
    }
    pub fn update(&mut self, data: resources_query::ResponseData) {
        self.feathers = data.village.feathers;
        self.sticks = data.village.sticks;
        self.logs = data.village.logs;
    }
    pub fn non_zero_resources(&self) -> Vec<(ResourceType, i64)> {
        use paddlers_shared_lib::strum::IntoEnumIterator;
        ResourceType::iter()
            .map(|rt| (rt, self.read(rt)))
            .filter( |t| t.1 > 0 )
            .collect()
    }
    fn spend_res(&mut self, rt: ResourceType, amount: i64) {
        *self.write(rt) -= amount;
    }
    pub fn spend(&mut self, p: &Price) {
        for (rt, n) in p.0.iter() {
            self.spend_res(*rt, *n);
        }
    }
    pub fn can_afford(&self, p: &Price) -> bool {
        for (rt, n) in p.0.iter() {
            if self.read(*rt) < *n {
                return false;
            }
        }
        true
    }
}