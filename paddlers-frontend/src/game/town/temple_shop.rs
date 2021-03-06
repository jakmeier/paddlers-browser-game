use crate::net::state::current_village;
use crate::prelude::*;
use crate::{
    game::player_info::PlayerState,
    gui::{
        gui_components::{ClickOutput, UiBox, UiElement},
        sprites::{SingleSprite, SpriteSet},
    },
};
use crate::{
    game::{components::UiMenu, game_event_manager::*, player_info::PlayerInfo, Game},
    net::game_master_api::RestApiState,
};
use paddle::quicksilver_compat::Color;
use paddlers_shared_lib::api::shop::ProphetPurchase;
use specs::prelude::*;

pub fn new_temple_menu(player_info: &PlayerInfo, has_quests: bool) -> UiMenu {
    let mut menu = UiMenu::new_private(UiBox::new(1, 2, 5.0, 5.0));
    if has_quests {
        menu.ui.add(
            UiElement::new(ClickOutput::Event(GameEvent::ToggleBetweenViews(
                UiView::Quests,
                UiView::Town,
            )))
            .with_image(SpriteSet::Simple(SingleSprite::Duties))
            .with_background_color(Color::BLACK),
        );
    }
    if player_info.civilization_perks().has_any() {
        menu.ui.add(
            UiElement::new(ClickOutput::Event(GameEvent::SwitchToView(
                UiView::Religion,
            )))
            .with_image(SpriteSet::Simple(SingleSprite::ReligionDroplets))
            .with_background_color(Color::BLACK),
        );
    }
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
        let player_info = self.world.fetch::<PlayerState>();
        let has_quests = true; // TODO: Actually check
        if let Some(temple) =
            super::Town::find_building_entity(&self.town_context.home_world(), BuildingType::Temple)
        {
            let mut menus = self.town_context.home_world().write_storage::<UiMenu>();
            // This insert overwrites existing entries
            menus
                .insert(temple, new_temple_menu(player_info.info(), has_quests))
                .map_err(|_| {
                    PadlError::dev_err(PadlErrorCode::EcsError("Temple menu insertion failed"))
                })?;
        }
        Ok(())
    }
}
