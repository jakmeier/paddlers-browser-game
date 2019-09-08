pub struct VillageMetaInfo {
    pub coordinates: (usize,usize),
}

use crate::net::graphql::query_types::map_query::*;

impl From<MapQueryMapVillages> for VillageMetaInfo {
    fn from(village: MapQueryMapVillages) -> Self {
        VillageMetaInfo {
            coordinates: (village.x as usize, village.y as usize)
        }
    }
}