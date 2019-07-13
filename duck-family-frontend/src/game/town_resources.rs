use duck_family_api_lib::types::*;
use crate::net::graphql::resources_query;

#[derive(Default,Debug)]
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
    pub fn update(&mut self, data: resources_query::ResponseData) {
        self.feathers = data.village.feathers;
        self.sticks = data.village.sticks;
        self.logs = data.village.logs;
    }
    pub fn non_zero_resources(&self) -> Vec<(ResourceType, i64)> {
        use duck_family_api_lib::strum::IntoEnumIterator;
        ResourceType::iter()
            .map(|rt| (rt, self.read(rt)))
            .filter( |t| t.1 > 0 )
            .collect()
    } 
}