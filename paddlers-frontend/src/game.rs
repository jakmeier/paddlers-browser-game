pub(crate) mod abilities;
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
pub(crate) mod visits;

use crate::game::{components::*, player_info::PlayerInfo, town::TownContext};
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
use shred::{Fetch, FetchMut};
use specs::prelude::*;
use std::sync::mpsc::{channel, Receiver};
use town::{DefaultShop, Town};

pub(crate) struct Game<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
    pub sprites: Sprites,
    pub locale: TextDb,
    pub net: Receiver<NetMsg>,
    pub time_zero: Timestamp,
    pub total_updates: u64,
    pub async_err_receiver: Receiver<PadlError>,
    pub game_event_receiver: Receiver<GameEvent>,
    pub event_pool: EventPool,
    pub stats: Statistician,
    // TODO: [0.1.4] These would better fit into frame state, however,
    // there is currently no good solution to share it between main-frame and menu-frame
    pub map: Option<GlobalMapPrivateState>,
    pub town_context: TownContext<'a, 'b>,

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
        let town_context = TownContext::new(
            resolution,
            game_evt_send.clone(),
            player_info.clone(),
            base.async_err.clone(),
        );
        let mut world =
            crate::init::init_world(base.async_err, resolution, player_info, base.errq, base.tb);
        let mut dispatcher = DispatcherBuilder::new()
            .with(RestApiSystem, "rest", &[])
            .build();
        dispatcher.setup(&mut world);

        let now = utc_now();
        world.insert::<Now>(Now(now));

        world.maintain();
        let mut game = Game {
            dispatcher: dispatcher,
            world: world,
            sprites,
            locale,
            net: base.net_chan,
            time_zero: now,
            total_updates: 0,
            async_err_receiver: base.err_recv,
            game_event_receiver: game_evt_recv,
            event_pool: game_evt_send,
            stats: Statistician::new(now),
            map: None,
            town_context,
            #[cfg(feature = "dev_view")]
            palette: false,
            #[cfg(feature = "dev_view")]
            active_test: None,
        };
        game.load_village_info(game_data.village_info.ok_or("No village info")?)?;
        game.load_buildings_from_net_response(
            game_data
                .buildings_response
                .ok_or("No buildings response")?,
        )?;
        // Make sure buildings are loaded properly before inserting any types of units
        game.world.maintain();
        game.town_world_mut().maintain();
        game.load_workers_from_net_response(game_data.worker_response.ok_or("No worker response")?);
        game.load_hobos_from_net_response(game_data.hobos_response.ok_or("No hobos response")?)?;
        game.load_attacking_hobos(game_data.attacking_hobos.ok_or("No attacks response")?)?;
        game.load_player_info(game_data.player_info.ok_or("No player info loaded")?)?;
        // Make sure all units are loaded properly before story triggers are added
        game.world.maintain();
        game.town_world_mut().maintain();
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

        let (button_area, inner_area) = crate::gui::menu::menu_box_inner_split(menu_area, r);

        let mut data = self.world.write_resource::<ViewState>();
        (*data).main_area = main_area;
        (*data).menu_box_area = menu_area;
        (*data).inner_menu_box_area = inner_area;
        (*data).button_area = button_area;

        // TODO: refresh map and town (and make this method callable by user input)
    }
    pub fn init_map(&mut self) {
        let main_area = self.world.read_resource::<ViewState>().main_area;
        let (private, shared) = GlobalMap::new(main_area.size());
        self.map = Some(private);
        self.world.insert(shared);
    }

    pub fn town(&self) -> Fetch<Town> {
        self.town_context.town()
    }
    pub fn town_mut(&self) -> FetchMut<Town> {
        self.town_context.town_mut()
    }
    pub fn map_mut(&mut self) -> GlobalMap {
        GlobalMap::combined(self.map.as_mut().unwrap(), self.world.write_resource())
    }
    pub fn player(&self) -> specs::shred::Fetch<PlayerInfo> {
        self.world.read_resource()
    }
    pub fn update_time_reference(&mut self) {
        if self.time_zero.micros() != 0 {
            let t = utc_now();
            let mut ts = self.world.write_resource::<Now>();
            *ts = Now(t);
        }
    }
    /// Removes entities outside the map
    pub fn reaper(&mut self, map: &Rectangle) {
        let p = self.town_world().read_storage::<Position>();
        let mut dead = vec![];
        for (entity, position) in (&self.town_world().entities(), &p).join() {
            if !position.area.overlaps_rectangle(map) {
                dead.push(entity);
            }
        }
        std::mem::drop(p);
        self.town_world_mut()
            .delete_entities(&dead)
            .expect("Something bad happened when deleting dead entities");
    }
    /// Deletes all building entities (lazy, requires world.maintain())
    fn flush_buildings(&self) -> PadlResult<()> {
        let b = self
            .town_context
            .town_world
            .read_storage::<buildings::Building>();
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
        let w = self.town_world().read_storage::<units::workers::Worker>();
        for (entity, _marker) in (&self.town_world().entities(), &w).join() {
            self.town_world()
                .entities()
                .delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::EcsError("Delete worker")))?;
        }
        Ok(())
    }
    /// Deletes all home hobo entities (lazy, requires world.maintain())
    fn flush_home_hobos(&self) -> PadlResult<()> {
        let world = self.town_world();
        let w = world.read_storage::<units::hobos::Hobo>();
        for (entity, _marker) in (&world.entities(), &w).join() {
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
        let world = self.town_world();
        let net = world.read_storage::<NetObj>();
        let ent = world.entities();
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
