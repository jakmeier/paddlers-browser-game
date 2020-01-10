pub (crate) mod abilities;
pub (crate) mod attacks;
pub (crate) mod buildings;
pub (crate) mod components;
pub (crate) mod fight;
pub (crate) mod forestry;
pub (crate) mod game_event_manager;
pub (crate) mod leaderboard;
pub (crate) mod level;
pub (crate) mod mana;
pub (crate) mod map;
pub (crate) mod movement;
pub (crate) mod net_receiver;
pub (crate) mod player_info;
pub (crate) mod status_effects;
pub (crate) mod town;
pub (crate) mod town_resources;
pub (crate) mod units;
#[cfg(feature="dev_view")]
pub (crate) mod dev_view;

use std::sync::mpsc::{channel, Receiver};
use specs::prelude::*;
use crate::prelude::*;
use crate::game::{
    components::*,
    fight::*,
    units::worker_system::WorkerSystem,
    forestry::ForestrySystem,
    player_info::PlayerInfo,
};
use crate::gui::{
    input,
    ui_state::*,
    sprites::*,
};
use crate::net::{
    NetMsg, 
    game_master_api::{RestApiState, RestApiSystem},
};
use crate::logging::{
    ErrorQueue,
    statistics::Statistician,
    text_to_user::TextBoard,
};
use movement::*;
use quicksilver::prelude::*;
use town::{Town, DefaultShop};
use town_resources::TownResources;
use map::{GlobalMap, GlobalMapPrivateState};
use game_event_manager::GameEvent;
use crate::view::FloatingText;

pub(crate) struct Game<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
    pub world: World,
    pub sprites: Option<Sprites>,
    pub resources: TownResources,
    pub net: Option<Receiver<NetMsg>>,
    pub time_zero: Timestamp,
    pub total_updates: u64,
    pub async_err_receiver: Receiver<PadlError>,
    pub game_event_receiver: Receiver<GameEvent>,
    pub stats: Statistician,
    pub map: Option<GlobalMapPrivateState>,

    pub preload: Option<crate::init::loading::LoadingState>,
    pub preload_float: FloatingText,

    #[cfg(feature="dev_view")]
    pub palette: bool,
    #[cfg(feature="dev_view")]
    pub active_test: Option<Box<crate::game::dev_view::benchmark::TestData>>,
}

impl Game<'_,'_> {

    pub fn load_game() -> PadlResult<(Self, EventPool)> {
        let (err_send, err_recv) = channel();
        let (game_evt_send, game_evt_recv) = channel();
        let mut world = crate::init::init_world(err_send);
        
        let mut dispatcher = DispatcherBuilder::new()
            .with(WorkerSystem::new(game_evt_send.clone()), "work", &[])
            .with(MoveSystem, "move", &["work"])
            .with(FightSystem::new(game_evt_send.clone()), "fight", &["move"])
            .with(ForestrySystem, "forest", &[])
            .with(RestApiSystem, "rest", &[])
            .build();
        dispatcher.setup(&mut world);

        let now = utc_now();

        Ok((Game {
            dispatcher: dispatcher,
            world: world,
            sprites: None,
            preload: Some(crate::init::loading::LoadingState::new()),
            preload_float: FloatingText::try_default().expect("FloatingText"),
            net: None,
            time_zero: now,
            resources: TownResources::default(),
            total_updates: 0,
            async_err_receiver: err_recv,
            game_event_receiver: game_evt_recv,
            stats: Statistician::new(now),
            map: None,
            #[cfg(feature="dev_view")]
            palette: false,
            #[cfg(feature="dev_view")]
            active_test: None,
        }, game_evt_send))
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
        self.handle_game_events();
        if self.total_updates % 300 == 15 {
            self.reaper(&Rectangle::new_sized(window.project() * window.screen_size()));
        }
        self.world.maintain();
        Ok(())
    }

    pub fn with_town(mut self, town: Town) -> Self {
        self.world.insert(town);
        self
    }
    pub fn with_resolution(mut self, r: ScreenResolution) -> Self {
        self.world.insert(r);
        self
    }
    pub fn with_network_chan(mut self, net_receiver: Receiver<NetMsg>) -> Self {
        self.net = Some(net_receiver);
        self
    }
    /// Call this after changing resolution in world
    pub fn load_resolution(&mut self) {

        let r = *self.world.fetch::<ScreenResolution>();

        let main_size = Vector::from(r.main_area());
        let menu_size = Vector::from(r.menu_area());
        let main_area = Rectangle::new_sized(main_size);
        let menu_area = Rectangle::new((main_size.x,0),menu_size);

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
        GlobalMap::combined(
            self.map.as_mut().unwrap(),
            self.world.write_resource(),
        )
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
            if !position.area.overlaps_rectangle(map)  {
                dead.push(entity);
            }
        }
        std::mem::drop(p);
        self.world.delete_entities(&dead).expect("Something bad happened when deleting dead entities");
    }
    /// Deletes all building entities (lazy, requires world.maintain())
    fn flush_buildings(&self) -> PadlResult<()> {
        let b = self.world.read_storage::<buildings::Building>();
        for (entity, _marker) in (&self.world.entities(), &b).join() {
            self.world.entities().delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::SpecsError("Delete building")))?;
        }
        Ok(())
    }
    /// Deletes all worker entities (lazy, requires world.maintain())
    fn flush_workers(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<units::workers::Worker>();
        for (entity, _marker) in (&self.world.entities(), &w).join() {
            self.world.entities().delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::SpecsError("Delete worker")))?;
        }
        Ok(())
    }
    /// Deletes all home hobo entities (lazy, requires world.maintain())
    fn flush_home_hobos(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<units::hobos::Hobo>();
        for (entity, _marker) in (&self.world.entities(), &w).join() {
            self.world.entities().delete(entity)
                .map_err(|_| PadlError::dev_err(PadlErrorCode::SpecsError("Delete hobo")))?;
        }
        Ok(())
    }
    /// Deletes all hobo entities (lazy, requires world.maintain())
    #[allow(dead_code)]
    fn flush_hobos(&self) -> PadlResult<()> {
        let w = self.world.read_storage::<components::NetObj>();
        for (entity, netid) in (&self.world.entities(), &w).join() {
            if netid.is_hobo() {
                self.world.entities().delete(entity)
                    .map_err(|_| PadlError::dev_err(PadlErrorCode::SpecsError("Delete hobo")))?;
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
        }
        else {
            Some(res.unwrap())
        }
    }
    pub fn confirm_to_user(&mut self, msg: String) -> PadlResult<()> {
        let mut tb = self.world.write_resource::<TextBoard>();
        tb.display_confirmation(msg)
    }
}
