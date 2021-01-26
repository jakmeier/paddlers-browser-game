use crate::game::{player_info::PlayerInfo, town::Town};
use crate::init::specs_registration::*;
use crate::prelude::*;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

/// Orchestrates TownContexts to be displayed.
///
/// At the moment, only the main town of the player currently logged in and one foreign town are stored.
/// In the future, this could also hold a cache of several villages to ensure quick loading when clicking through many towns and going back to previously visited ones.
pub struct TownContextManager {
    home_town: TownContext,
    foreign_town: Option<TownContext>,
}
/// Data structure holding all info that is specific to one town.
/// This can be swapped out to display another town, including foreign towns.
pub struct TownContext {
    id: VillageKey,
    pub town_world: World,
}

impl TownContextManager {
    /// Create the TownContextManager and load the home town into it
    pub fn new(player_info: PlayerInfo) -> Self {
        let vid = crate::net::state::current_village();
        Self {
            home_town: TownContext::new(player_info, vid, false),
            foreign_town: None,
        }
    }
    /// Load a new town context for a foreign town
    pub fn load_foreign(&mut self, v: VillageKey) {
        let home_data = self.home_town.world();
        let player_info = *home_data.fetch::<PlayerInfo>();
        self.foreign_town = Some(TownContext::new(player_info, v, true));
    }
    /// Remove all loaded foreign towns from the view and display home again
    pub fn reset_to_home(&mut self) {
        self.foreign_town = None;
    }
    /// Return true if there is currently a foreign town being displayed
    pub fn is_foreign(&self) -> bool {
        self.foreign_town.is_some()
    }

    pub fn context_by_key_mut(&mut self, vid: VillageKey) -> Option<&mut TownContext> {
        if self.home_town.id == vid {
            Some(&mut self.home_town)
        } else {
            self.foreign_town.as_mut().filter(|v| v.id == vid)
        }
    }
    pub fn active_context(&self) -> &TownContext {
        if let Some(ctx) = self.foreign_town.as_ref() {
            ctx
        } else {
            &self.home_town
        }
    }
    pub fn active_context_mut(&mut self) -> &mut TownContext {
        if let Some(ctx) = self.foreign_town.as_mut() {
            ctx
        } else {
            &mut self.home_town
        }
    }
    pub fn world(&self) -> &World {
        self.active_context().world()
    }
    pub fn world_mut(&mut self) -> &mut World {
        self.active_context_mut().world_mut()
    }
    pub fn home_world(&self) -> &World {
        self.home_town.world()
    }
    pub fn home_world_mut(&mut self) -> &mut World {
        self.home_town.world_mut()
    }
    pub fn town(&self) -> specs::shred::Fetch<Town> {
        self.active_context().town()
    }
    pub fn town_mut(&self) -> specs::shred::FetchMut<Town> {
        self.active_context().town_mut()
    }
}

impl TownContext {
    fn new(player_info: PlayerInfo, vid: VillageKey, foreign: bool) -> Self {
        let mut world = World::new();
        register_town_components(&mut world);

        let town = Town::new(foreign);
        insert_town_resources(&mut world, player_info, town);

        Self {
            town_world: world,
            id: vid,
        }
    }
    pub fn world(&self) -> &World {
        &self.town_world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.town_world
    }
    pub fn town(&self) -> specs::shred::Fetch<Town> {
        self.town_world.fetch()
    }
    pub fn town_mut(&self) -> specs::shred::FetchMut<Town> {
        self.town_world.fetch_mut()
    }
}

impl Game {
    pub fn town_world(&self) -> &World {
        self.town_context.world()
    }
    pub fn town_world_mut(&mut self) -> &mut World {
        self.town_context.world_mut()
    }
}

impl std::fmt::Debug for TownContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("village id", &self.id)
            .finish()
    }
}
