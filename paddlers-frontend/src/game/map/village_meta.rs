use specs::prelude::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct VillageMetaInfo {
    pub coordinates: (i32,i32),
}

use crate::net::graphql::query_types::map_query::*;

impl From<MapQueryMapVillages> for VillageMetaInfo {
    fn from(village: MapQueryMapVillages) -> Self {
        VillageMetaInfo {
            coordinates: (village.x as i32, village.y as i32)
        }
    }
}