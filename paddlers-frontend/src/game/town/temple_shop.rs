use crate::game::{
    buildings::Building, components::UiMenu, game_event_manager::*, player_info::PlayerInfo, Game,
};
use crate::gui::sprites::{SingleSprite, SpriteSet};
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;
use crate::prelude::*;
use paddlers_shared_lib::api::shop::ProphetPurchase;
use specs::prelude::*;

pub fn new_temple_menu(player_info: &PlayerInfo) -> UiMenu {
    UiMenu::new_shop_menu().with_shop_item(
        GameEvent::HttpBuyProphet,
        SpriteSet::Simple(SingleSprite::Prophet),
        player_info.prophet_price(),
    )
}

pub fn purchase_prophet(player_info: &PlayerInfo) -> PadlResult<()> {
    if player_info.prophets_limit() <= player_info.prophets_total() {
        return PadlErrorCode::NotEnoughKarma.usr();
    }
    RestApiState::get().http_buy_prophet(ProphetPurchase {
        village: current_village(),
    })?;
    Ok(())
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn update_temple(&self) -> PadlResult<()> {
        let player_info = self.world.fetch::<PlayerInfo>();
        if let Some(temple) = find_temple(&self.town_context.home_world()) {
            let mut menus = self.town_context.home_world().write_storage::<UiMenu>();
            // This insert overwrites existing entries
            menus
                .insert(temple, new_temple_menu(&player_info))
                .map_err(|_| {
                    PadlError::dev_err(PadlErrorCode::EcsError("Temple menu insertion failed"))
                })?;
        }
        Ok(())
    }
}

fn find_temple(world: &World) -> Option<Entity> {
    let buildings = world.read_component::<Building>();
    let entities = world.entities();
    for (b, e) in (&buildings, &entities).join() {
        if b.bt == BuildingType::Temple {
            return Some(e);
        }
    }
    None
}
