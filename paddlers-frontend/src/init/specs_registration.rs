use crate::prelude::*;
use specs::prelude::*;

use crate::game::{components::*, town_resources::TownResources, units::hobos::Hobo, units::workers::Worker, player_info::PlayerInfo, attacks::Attack};
use crate::gui::{menu::buttons::MenuButtons, ui_state::*};
use crate::logging::{text_to_user::TextBoard, AsyncErr, ErrorQueue};
use crate::net::game_master_api::RestApiState;

pub(super) fn insert_resources(world: &mut World, err_send: std::sync::mpsc::Sender<PadlError>) {
    let err_send_clone = err_send.clone();
    world.insert(AsyncErr::new(err_send));
    world.insert(ClockTick(0));
    world.insert(ErrorQueue::default());
    world.insert(MenuButtons::new());
    world.insert(Now);
    world.insert(PlayerInfo::default());
    world.insert(RestApiState::new(err_send_clone));
    world.insert(TextBoard::default());
    world.insert(TownResources::default());
    world.insert(UiState::default());
}

pub fn register_components(world: &mut World) {
    world.register::<AnimationState>();
    world.register::<Attack>();
    world.register::<Building>();
    world.register::<Clickable>();
    world.register::<EntityContainer>();
    world.register::<ForestComponent>();
    world.register::<Health>();
    world.register::<Hobo>();
    world.register::<Level>();
    world.register::<Mana>();
    world.register::<MapPosition>();
    world.register::<Moving>();
    world.register::<NetObj>();
    world.register::<Position>();
    world.register::<Range>();
    world.register::<Renderable>();
    world.register::<StatusEffects>();
    world.register::<UiMenu>();
    world.register::<VillageMetaInfo>();
    world.register::<Worker>();
}
