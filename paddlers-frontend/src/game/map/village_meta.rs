use specs::storage::BTreeStorage;
use specs::prelude::*;
use crate::gui::gui_components::TableRow;

#[derive(Component, Debug, Clone)]
#[storage(BTreeStorage)]
pub struct VillageMetaInfo {
    pub coordinates: (i32,i32),
    player: Option<PlayerMetaInfo>,
}

#[derive(Debug, Clone)]
struct PlayerMetaInfo {
    name: String,
    karma: i64,
}

use crate::net::graphql::query_types::map_query::*;

impl From<MapQueryMapVillages> for VillageMetaInfo {
    fn from(village: MapQueryMapVillages) -> Self {
        let player = village.owner.map(
            |p|
            PlayerMetaInfo {
                name: p.display_name,
                karma: p.karma,
            }
        );
        VillageMetaInfo {
            coordinates: (village.x as i32, village.y as i32),
            player
        }
    }
}

impl VillageMetaInfo {
    pub fn village_details<'a>(&self) -> Vec<TableRow<'a>> {
        let text = format!("Village <{}:{}>", self.coordinates.0, self.coordinates.1);
        let row0 = TableRow::Text(text);
        let row1 = self.player_info_row();
        vec![row0, row1]
    }
    fn player_info_row<'a>(&self) -> TableRow<'a> {
        let text =
        if let Some(p) = &self.player {
            format!("{} ({})", p.name, p.karma)
        } else {
            "Anarchists".to_owned()
        };
        TableRow::Text(text)
    }
}