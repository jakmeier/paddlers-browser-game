use crate::gui::gui_components::{ClickOutput, UiElement};
use crate::net::state::current_village;
use crate::prelude::*;
use crate::{
    game::{components::UiMenu, game_event_manager::*, player_info::PlayerInfo, Game},
    net::game_master_api::RestApiState,
};
use paddle::quicksilver_compat::Color;
use paddlers_shared_lib::api::shop::ProphetPurchase;
use specs::prelude::*;

pub fn new_temple_menu(_player_info: &PlayerInfo) -> UiMenu {
    let mut menu = UiMenu::new_shop_menu()
    // No prophets for now
    // .with_shop_item(
    //     GameEvent::HttpBuyProphet,
    //     SpriteSet::Simple(SingleSprite::Prophet),
    //     player_info.prophet_price(),
    // )
    ;
    menu.ui.add(
        UiElement::new(ClickOutput::Event(GameEvent::SwitchToView(
            UiView::Religion,
        )))
        .with_background_color(Color::BLACK),
    );
    menu
}

pub fn purchase_prophet(player_info: &PlayerInfo) -> PadlResult<()> {
    if player_info.prophets_limit() <= player_info.prophets_total() {
        return PadlErrorCode::NotEnoughKarma(player_info.karma_for_next_prophet()).usr();
    }
    nuts::send_to::<RestApiState, _>(ProphetPurchase {
        village: current_village(),
    });
    Ok(())
}

impl Game {
    pub fn update_temple(&self) -> PadlResult<()> {
        let player_info = self.world.fetch::<PlayerInfo>();
        if let Some(temple) =
            super::Town::find_building(&self.town_context.home_world(), BuildingType::Temple)
        {
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
