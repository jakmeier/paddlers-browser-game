use crate::game::{
    components::*,
    fight::Aura,
    player_info::PlayerInfo,
    story::entity_trigger::EntityTrigger,
    town::DefaultShop,
    town::Town,
    town::{nests::Nest, visitor_gate::VisitorGate},
    town_resources::TownResources,
    units::attackers::Visitor,
    units::hobos::Hobo,
    units::workers::Worker,
    visits::attacks::Attack,
};
use crate::gui::ui_state::*;
use crate::view::entry_view;
use paddle::utc_now;
use specs::prelude::*;

pub(super) fn insert_global_resources(world: &mut World, player_info: PlayerInfo) {
    world.insert(ClockTick(0));
    world.insert(Now(utc_now()));
    world.insert(UiState::new());
    world.insert(player_info);
    let view = entry_view(player_info.story_state());
    world.insert(view);
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

pub fn insert_town_resources(world: &mut World, player_info: PlayerInfo, town: Town) {
    world.insert(DefaultShop::new(&player_info));
    world.insert(Now(utc_now()));
    world.insert(TownResources::default());
    world.insert(UiState::new());
    world.insert(player_info);
    world.insert(town);
    world.insert(VisitorGate::new());
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
    world.register::<Nest>();
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
    world.register::<ForeignUiMenu>();
}
fn register_graphic_components(world: &mut World) {
    world.register::<AnimationState>();
    world.register::<Position>();
    world.register::<Renderable>();
}
