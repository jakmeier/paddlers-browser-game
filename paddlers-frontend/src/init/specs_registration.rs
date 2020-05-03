use crate::game::story::entity_trigger::EntityTrigger;
use crate::prelude::*;
use crate::view::entry_view;
use specs::prelude::*;

use crate::game::town::DefaultShop;
use crate::game::{
    components::*, player_info::PlayerInfo, town::Town, town_resources::TownResources,
    units::hobos::Hobo, units::workers::Worker, visits::attacks::Attack,
};
use crate::gui::ui_state::*;
use crate::logging::{text_to_user::TextBoard, AsyncErr, ErrorQueue};

pub(super) fn insert_resources(
    world: &mut World,
    async_err: AsyncErr,
    resolution: ScreenResolution,
    player_info: PlayerInfo,
    errq: ErrorQueue,
    tb: TextBoard,
) {
    world.insert(async_err);
    world.insert(ClockTick(0));
    world.insert(DefaultShop::new(&player_info));
    world.insert(errq);
    world.insert(Now);
    world.insert(resolution);
    world.insert(tb);
    world.insert(Town::new(resolution));
    world.insert(TownResources::default());
    world.insert(player_info);
    let view = entry_view(player_info.story_state());
    world.insert(UiState::new(view));
    // TODO: Only temporary experiment
    world.insert(crate::view::ExperimentalSignalChannel::new());
}

pub fn register_components(world: &mut World) {
    world.register::<AnimationState>();
    world.register::<Attack>();
    world.register::<Building>();
    world.register::<Clickable>();
    world.register::<EntityContainer>();
    world.register::<EntityTrigger>();
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
