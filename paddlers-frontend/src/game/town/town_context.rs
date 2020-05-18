use crate::game::{
    fight::*, forestry::ForestrySystem, movement::MoveSystem, player_info::PlayerInfo,
    story::entity_trigger::EntityTriggerSystem, town::Town,
    units::worker_system::WorkerSystem,
};
use crate::init::specs_registration::*;
use crate::logging::AsyncErr;
use crate::prelude::*;
use specs::prelude::*;

/// Data structure holding all info that is specific to one town.
/// This can be swapped out to display another town, including foreign towns.
pub struct TownContext<'a, 'b> {
    pub town_world: World,
    // pub town_resources: TownResources,
    // FIXME: This should be shared between towns, no need to have different dispatchers
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> TownContext<'a, 'b> {
    pub fn new(
        resolution: ScreenResolution,
        game_evt_send: EventPool,
        player_info: PlayerInfo,
        async_err: AsyncErr,
    ) -> Self {
        let mut world = World::new();
        register_town_components(&mut world);

        let town = Town::new(resolution);
        insert_town_resources(&mut world, player_info, async_err, town);

        let mut dispatcher = DispatcherBuilder::new()
            .with(WorkerSystem::new(game_evt_send.clone()), "work", &[])
            .with(MoveSystem, "move", &["work"])
            .with(FightSystem::new(game_evt_send.clone()), "fight", &["move"])
            .with(ForestrySystem, "forest", &[])
            .with(EntityTriggerSystem::new(game_evt_send.clone()), "ets", &[])
            .build();

        dispatcher.setup(&mut world);

        Self {
            town_world: world,
            // town_resources: TownResources::default(),
            dispatcher,
        }
    }
    pub fn world(&self) -> &World {
        &self.town_world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.town_world
    }
    pub fn update(&mut self) {
        self.town_world.maintain();
        self.dispatcher.dispatch(&mut self.town_world)
    }
    pub fn town(&self) -> specs::shred::Fetch<Town> {
        self.town_world.fetch()
    }
    pub fn town_mut(&self) -> specs::shred::FetchMut<Town> {
        self.town_world.fetch_mut()
    }
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn town_world(&self) -> &World {
        self.town_context.world()
    }
    pub fn town_world_mut(&mut self) -> &mut World {
        self.town_context.world_mut()
    }
}
