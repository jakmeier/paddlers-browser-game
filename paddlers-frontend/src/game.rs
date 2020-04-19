pub(crate) mod abilities;
pub(crate) mod attacks;
pub(crate) mod buildings;
pub(crate) mod components;
#[cfg(feature = "dev_view")]
pub(crate) mod dev_view;
pub(crate) mod dialogue;
pub(crate) mod fight;
pub(crate) mod forestry;
pub(crate) mod game_event_manager;
pub(crate) mod leaderboard;
pub(crate) mod level;
pub(crate) mod mana;
pub(crate) mod map;
pub(crate) mod movement;
pub(crate) mod net_receiver;
pub(crate) mod player_info;
pub(crate) mod status_effects;
pub(crate) mod story;
pub(crate) mod town;
pub(crate) mod town_resources;
pub(crate) mod units;

use crate::game::town::new_temple_menu;
use crate::game::{
    components::*, fight::*, forestry::ForestrySystem, player_info::PlayerInfo,
    story::entity_trigger::EntityTriggerSystem, units::worker_system::WorkerSystem,
};
use crate::gui::{input, sprites::*, ui_state::*};
use crate::init::loading::BaseState;
use crate::init::loading::GameLoadingData;
use crate::logging::{statistics::Statistician, text_to_user::TextBoard, ErrorQueue};
use crate::net::{
    game_master_api::{RestApiState, RestApiSystem},
    NetMsg,
};
use crate::prelude::*;
use game_event_manager::GameEvent;
use map::{GlobalMap, GlobalMapPrivateState};
use movement::*;
use quicksilver::prelude::*;
use specs::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use town::{DefaultShop, Town};
use town_resources::TownResources;

pub(crate) struct Game<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
    pub sprites: Sprites,
    pub locale: TextDb,
    pub resources: TownResources,
    pub net: Receiver<NetMsg>,
    pub time_zero: Timestamp,
    pub total_updates: u64,
    pub async_err_receiver: Receiver<PadlError>,
    pub game_event_receiver: Receiver<GameEvent>,
    pub event_pool: EventPool,
    pub stats: Statistician,
    pub map: Option<GlobalMapPrivateState>,

    #[cfg(feature = "dev_view")]
    pub palette: bool,
    #[cfg(feature = "dev_view")]
    pub active_test: Option<Box<crate::game::dev_view::benchmark::TestData>>,
}

impl Game<'_, '_> {
    pub fn load_game(
        sprites: Sprites,
        locale: TextDb,
        resolution: ScreenResolution,
        game_data: GameLoadingData,
        base: BaseState,
    ) -> PadlResult<Self> {
        let (game_evt_send, game_evt_recv) = channel();
        let player_info = game_data.player_info.ok_or("Player Info not loaded")?; 
        let mut world = crate::init::init_world(
            base.err_send,
            resolution,
            player_info,
            base.rest,
            base.errq,
            base.tb,
        );
        let mut dispatcher = DispatcherBuilder::new()
            .with(WorkerSystem::new(game_evt_send.clone()), "work", &[])
            .with(MoveSystem, "move", &["work"])
            .with(FightSystem::new(game_evt_send.clone()), "fight", &["move"])
            .with(ForestrySystem, "forest", &[])
            .with(RestApiSystem, "rest", &[])
            .with(EntityTriggerSystem::new(game_evt_send.clone()), "ets", &[])
            .build();
        dispatcher.setup(&mut world);

        let now = utc_now();

        if let Some(temple) = world.read_resource::<Town>().temple {
            let mut menus = world.write_storage::<UiMenu>();
            // This insert overwrites existing entries
            menus
                .insert(temple, new_temple_menu(&player_info))
                .map_err(|_| {
                    PadlError::dev_err(PadlErrorCode::EcsError("Temple menu insertion failed"))
                })?;
        }
        world.maintain();
        let mut game = Game {
            dispatcher: dispatcher,
            world: world,
            sprites,
            locale,
            net: base.net_chan,
            time_zero: now,
            resources: TownResources::default(),
            total_updates: 0,
            async_err_receiver: base.err_recv,
            game_event_receiver: game_evt_recv,
            event_pool: game_evt_send,
            stats: Statistician::new(now),
            map: None,
            #[cfg(feature = "dev_view")]
            palette: false,
            #[cfg(feature = "dev_view")]
            active_test: None,
        };
        game.load_workers_from_net_response(game_data.worker_response.ok_or("No worker response")?);
        game.world.maintain();
        game.load_story_state()?;
        Ok(game)
    }

    /// Called at the first draw loop iteration (the first time quicksilver leaks access to it)
    pub fn initialize_with_window(&mut self, window: &mut Window) {
        self.load_resolution();
        self.init_map();
        let err = crate::window::adapt_window_size(window);
        self.check(err);
    }

    pub fn main_update_loop(&mut self, window: &mut Window) -> Result<()> {
        {
            let mut res = self.world.write_resource::<TownResources>();
            *res = self.resources;
        }
        {
            self.map_mut().update();
        }
        self.update_time_reference();
        self.dispatcher.dispatch(&mut self.world);
        if self.total_updates % 300 == 15 {
            self.reaper(&Rectangle::new_sized(
                window.project() * window.screen_size(),
            ));
        }
        self.world.maintain();
        Ok(())
    }
    /// Call this after changing resolution in world
    pub fn load_resolution(&mut self) {
        let r = *self.world.fetch::<ScreenResolution>();

        let main_size = Vector::from(r.main_area());
        let menu_size = Vector::from(r.menu_area());
        let main_area = Rectangle::new_sized(main_size);
        let menu_area = Rectangle::new((main_size.x, 0), menu_size);

        let mut data = self.world.write_resource::<UiState>();
        (*data).main_area = main_area;
        (*data).menu_box_area = menu_area;
        std::mem::drop(data);

        // TODO: refresh map and town (and make this method callable by user input)
    }
    pub fn init_map(&mut self) {
        let main_area = self.world.read_resource::<UiState>().main_area;
        let (private, shared) = GlobalMap::new(main_area.size());
        self.map = Some(private);
        self.world.insert(shared);
    }

    pub fn town(&self) -> specs::shred::Fetch<Town> {
        self.world.read_resource()
    }
    pub fn town_mut(&mut self) -> specs::shred::FetchMut<Town> {
        self.world.write_resource()
    }
    pub fn map_mut(&mut self) -> GlobalMap {
        GlobalMap::combined(self.map.as_mut().unwrap(), self.world.write_resource())
    }
    pub fn rest(&mut self) -> specs::shred::FetchMut<RestApiState> {
        self.world.write_resource()
    }
    pub fn player(&self) -> specs::shred::Fetch<PlayerInfo> {
        self.world.read_resource()
    }
    pub fn update_time_reference(&mut self) {
        if self.time_zero != 0 {
            let t = utc_now();
            let mut ts = self.world.write_resource::<Now>();
            *ts = Now(t);
        }
    }
    /// Removes entities outside the map
    pub fn reaper(&mut self, map: &Rectangle) {
        let p = self.world.read_storage::<Position>();
        let mut dead = vec![];
        for (entity, position) in (&self.world.entities(), &p).join() {
            if !position.area.overlaps_rectangle(map) {
                dead.push(entity);
            }
        }
        std::mem::drop(p);
        self.world
            .delete_entities(&dead)
            .expect("Something bad happened when deleting dead entities");
    }
    /// Deletes all building entities (lazy, requires world.maintain())
    fn flush_buildings(&self) -> PadlResult<()> {
        let b = self.world.read_storage::<buildings::Building>();
        for (entity, _marker) in (&self.world.entities(), &b).join() {
            self.world
                .entities()
                .delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::EcsError("Delete building")))?;
        }
        Ok(())
    }
    /// Deletes all worker entities (lazy, requires world.maintain())
    fn flush_workers(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<units::workers::Worker>();
        for (entity, _marker) in (&self.world.entities(), &w).join() {
            self.world
                .entities()
                .delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::EcsError("Delete worker")))?;
        }
        Ok(())
    }
    /// Deletes all home hobo entities (lazy, requires world.maintain())
    fn flush_home_hobos(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<units::hobos::Hobo>();
        for (entity, _marker) in (&self.world.entities(), &w).join() {
            self.world
                .entities()
                .delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::EcsError("Delete hobo")))?;
        }
        Ok(())
    }
    /// Deletes all hobo entities (lazy, requires world.maintain())
    #[allow(dead_code)]
    fn flush_hobos(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<components::NetObj>();
        for (entity, netid) in (&self.world.entities(), &w).join() {
            if netid.is_hobo() {
                self.world
                    .entities()
                    .delete(entity)
                    .map_err(|_| PadlError::dev_err(PadlErrorCode::EcsError("Delete hobo")))?;
            }
        }
        Ok(())
    }
    fn worker_entity_by_net_id(&self, net_id: i64) -> PadlResult<Entity> {
        // TODO: Efficient NetId lookup
        let net = self.world.read_storage::<NetObj>();
        let ent = self.world.entities();
        NetObj::lookup_worker(net_id, &net, &ent)
    }
    /// Transforms a PadlResult to an Option, handing errors to the error queue
    pub fn check<R>(&self, res: PadlResult<R>) -> Option<R> {
        if let Err(e) = res {
            let mut q = self.world.write_resource::<ErrorQueue>();
            q.push(e);
            None
        } else {
            Some(res.unwrap())
        }
    }
    pub fn confirm_to_user(&mut self, msg: String) -> PadlResult<()> {
        let mut tb = self.world.write_resource::<TextBoard>();
        tb.display_confirmation(msg)
    }
}
