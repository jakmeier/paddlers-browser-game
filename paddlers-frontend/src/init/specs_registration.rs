use crate::game::fight::Aura;
use crate::game::story::entity_trigger::EntityTrigger;
use crate::game::units::attackers::Visitor;
use crate::prelude::*;
use crate::view::entry_view;
use specs::prelude::*;

use crate::game::town::DefaultShop;
use crate::game::{
    components::*, player_info::PlayerInfo, town::Town, town_resources::TownResources,
    units::hobos::Hobo, units::workers::Worker, visits::attacks::Attack,
};
use crate::gui::input::drag::Drag;
use crate::gui::ui_state::*;
use crate::logging::{text_to_user::TextBoard, AsyncErr, ErrorQueue};

pub(super) fn insert_global_resources(
    world: &mut World,
    async_err: AsyncErr,
    resolution: ScreenResolution,
    player_info: PlayerInfo,
    errq: ErrorQueue,
    tb: TextBoard,
) {
    world.insert(ClockTick(0));
    world.insert(Now(utc_now()));
    world.insert(UiState::new());
    world.insert(ViewState::new());
    world.insert(async_err);
    world.insert(errq);
    world.insert(player_info);
    world.insert(resolution);
    world.insert(tb);
    let view = entry_view(player_info.story_state());
    world.insert(view);
    // TODO [0.1.4]: Only temporary experiment
    world.insert(crate::view::ExperimentalSignalChannel::new());
}

pub fn register_global_components(world: &mut World) {
    world.register::<NetObj>();
    register_graphic_components(world);
    register_ui_components(world);

    // Map view
    world.register::<MapPosition>();
    world.register::<VillageMetaInfo>();

    // Visits view
    world.register::<Attack>();
}

pub fn insert_town_resources(
    world: &mut World,
    player_info: PlayerInfo,
    async_err: AsyncErr,
    town: Town,
) {
    world.insert(DefaultShop::new(&player_info));
    world.insert(Drag::default());
    world.insert(ErrorQueue::new_endpoint());
    world.insert(Now(utc_now()));
    world.insert(TownResources::default());
    world.insert(UiState::new());
    world.insert(ViewState::new());
    world.insert(async_err);
    world.insert(player_info);
    world.insert(town);

    // TODO [0.1.4]: Only temporary experiment
    world.insert(crate::view::ExperimentalSignalChannel::new());
}

/// All components used in the town view
pub fn register_town_components(world: &mut World) {
    world.register::<Aura>();
    world.register::<Building>();
    world.register::<EntityContainer>();
    world.register::<ForestComponent>();
    world.register::<Health>();
    world.register::<Hobo>();
    world.register::<Level>();
    world.register::<Mana>();
    world.register::<Moving>();
    world.register::<Range>();
    world.register::<StatusEffects>();
    world.register::<TargetPosition>();
    world.register::<Visitor>();
    world.register::<Worker>();

    register_graphic_components(world);
    register_ui_components(world);
    world.register::<NetObj>();
}

fn register_ui_components(world: &mut World) {
    world.register::<Clickable>();
    world.register::<EntityTrigger>();
    world.register::<UiMenu>();
}
fn register_graphic_components(world: &mut World) {
    world.register::<AnimationState>();
    world.register::<Position>();
    world.register::<Renderable>();
}
