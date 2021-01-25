use crate::game::components::UiMenu;
use crate::game::game_event_manager::VillageCoordinate;
use crate::game::GameEvent;
use crate::gui::gui_components::TableRow;
use crate::gui::{
    gui_components::{ClickOutput, UiBox, UiElement},
    sprites::*,
    utils::*,
};
use paddlers_shared_lib::prelude::VillageKey;
use specs::prelude::*;
use specs::storage::BTreeStorage;

#[derive(Component, Debug, Clone)]
#[storage(BTreeStorage)]
pub struct VillageMetaInfo {
    pub id: VillageKey,
    pub coordinates: VillageCoordinate,
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
        let player = village.owner.map(|p| PlayerMetaInfo {
            name: p.display_name,
            karma: p.karma,
        });
        VillageMetaInfo {
            id: VillageKey(village.id),
            coordinates: (village.x as i32, village.y as i32),
            player,
        }
    }
}

impl VillageMetaInfo {
    pub fn player_name(&self) -> Option<&str> {
        self.player.as_ref().map(|s| s.name.as_str())
    }
    pub fn village_details<'a>(&self) -> Vec<TableRow<'a>> {
        let text = format!("Village <{}:{}>", self.coordinates.0, self.coordinates.1);
        let row0 = TableRow::Text(text);
        let row1 = self.player_info_row();
        vec![row0, row1]
    }
    fn player_info_row<'a>(&self) -> TableRow<'a> {
        let text = if let Some(p) = &self.player {
            format!("{} ({})", p.name, p.karma)
        } else {
            "Anarchists".to_owned()
        };
        TableRow::Text(text)
    }
    pub fn new_village_menu(&self, owned: bool) -> UiMenu {
        let mut menu = UiMenu::new_public(UiBox::new(2, 2, 10.0, 2.0));
        if !owned {
            menu.ui.add(
                UiElement::new(ClickOutput::Event(GameEvent::LoadVillage(self.id)))
                    .with_text("Descend".to_owned())
                    .with_background_color(LIGHT_BLUE),
            );
            menu.ui.add(
                UiElement::new(ClickOutput::Event(GameEvent::SendProphetAttack(
                    self.coordinates,
                )))
                .with_image(SpriteSet::Simple(SingleSprite::Prophet))
                .with_background_color(RED),
            );
        }
        menu
    }
}
