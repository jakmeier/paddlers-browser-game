pub mod ajax;
pub mod authentication;
pub mod game_master_api;
pub mod graphql;
pub mod state;
pub mod url;

use crate::game::player_info::PlayerInfo;
use game_master_api::RestApiState;
use graphql::{query_types::*, GraphQlState};
use paddlers_shared_lib::prelude::VillageKey;
use std::sync::Arc;

use stdweb::spawn_local;

use futures::future::TryFutureExt;
use futures::Future;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Mutex,
};

use crate::prelude::*;

pub enum NetMsg {
    Attacks(AttacksResponse),
    Buildings(BuildingsResponse),
    Error(PadlError),
    Hobos(HobosQueryResponse, VillageKey),
    Leaderboard(usize, Vec<(String, i64)>),
    Map(MapResponse, i32, i32),
    Player(PlayerInfo),
    VillageInfo(VolatileVillageInfoResponse),
    UpdateWorkerTasks(WorkerTasksResponse),
    Workers(WorkerResponse, VillageKey),
    Reports(ReportsResponse),
}

pub enum NetUpdateRequest {
    CompleteReload,
    WorkerTasks(i64),
    PlayerInfo,
}

struct NetState {
    interval_ms: u32,
    logged_in: AtomicBool,
    game_ready: AtomicBool,
    chan: Option<Mutex<Sender<NetMsg>>>,
    gql_state: GraphQlState,
    rest: Option<Arc<Mutex<RestApiState>>>,
}
// FIXME: This is probably better packed in a thread_local!()
static mut STATIC_NET_STATE: NetState = NetState {
    interval_ms: 5_000,
    logged_in: AtomicBool::new(false),
    game_ready: AtomicBool::new(false),
    chan: None,
    gql_state: GraphQlState::new(),
    rest: None,
};

/// Initializes state necessary for networking
pub fn init_net(chan: Sender<NetMsg>) {
    unsafe {
        STATIC_NET_STATE.chan = Some(Mutex::new(chan));
        STATIC_NET_STATE.rest = Some(Arc::new(Mutex::new(RestApiState::new())));
    }
}
/// Sets up continuous networking with the help of JS setTimeout
pub fn activate_net() {
    unsafe {
        STATIC_NET_STATE.game_ready.store(true, Ordering::Relaxed);
        // Brute-force sync up with WASM initialization
        while STATIC_NET_STATE.chan.is_none() {
            // NOP
        }
    }
}
#[wasm_bindgen]
/// Sets up continuous networking with the help of JS setTimeout
/// Must be called from JS once the user is logged in
pub fn start_network_thread() {
    unsafe {
        STATIC_NET_STATE.logged_in.store(true, Ordering::Relaxed);
        // Brute-force sync up with WASM initialization
        while STATIC_NET_STATE.chan.is_none() {
            // NOP
        }
        STATIC_NET_STATE.work();
    }
}
/// Sends all requests out necessary for the client state to display a full game view including the home town
pub fn request_client_state() {
    unsafe {
        // TODO: Instead of forcing a request every time the 10s are too long, use something smarter.$
        // E.g.: Once a second check what needs to be updated and then allow this list to be altered from outside
        if STATIC_NET_STATE.logged_in.load(Ordering::Relaxed) {
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.buildings_query());
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.workers_query());
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.hobos_query());
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.leaderboard_query());
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.attacks_query());
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.resource_query());
            request_player_update();
        } else {
            stdweb::web::set_timeout(request_client_state, 10);
        }
    }
}
pub fn request_map_read(min: i32, max: i32) {
    unsafe {
        if STATIC_NET_STATE.logged_in.load(Ordering::Relaxed) {
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.map_query(min, max));
        }
    }
}
pub fn request_worker_tasks_update(unit_id: i64) {
    unsafe {
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.worker_tasks_query(unit_id));
    }
}
pub fn request_resource_update() {
    unsafe {
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.resource_query());
    }
}
pub fn request_player_update() {
    unsafe {
        if STATIC_NET_STATE.logged_in.load(Ordering::Relaxed) {
            STATIC_NET_STATE.spawn(GraphQlState::player_info_query());
        } else {
            stdweb::web::set_timeout(request_player_update, 10);
        }
    }
}
pub fn request_foreign_town(vid: VillageKey) {
    unsafe {
        STATIC_NET_STATE.spawn(Ok(STATIC_NET_STATE.gql_state.foreign_buildings_query(vid)));
        STATIC_NET_STATE.spawn(Ok(STATIC_NET_STATE.gql_state.foreign_hobos_query(vid)));
        // TODO: Other state
    }
}
impl NetState {
    fn register_networking(&'static self) {
        let ms = self.interval_ms;
        stdweb::web::set_timeout(move || self.work(), ms);
    }
    // For frequent updates
    fn work(&'static self) {
        if self.game_ready.load(Ordering::Relaxed) {
            self.spawn(self.gql_state.attacks_query());
            self.spawn(self.gql_state.reports_query());
            self.spawn(self.gql_state.resource_query());
            self.spawn(GraphQlState::player_info_query());
        }
        self.register_networking();
    }

    fn get_channel(&self) -> Sender<NetMsg> {
        match self.chan.as_ref().unwrap().lock() {
            Ok(chan) => chan.clone(),
            Err(e) => panic!("Could not get channel: {}.", e),
        }
    }

    fn spawn<Q: Future<Output = PadlResult<NetMsg>> + 'static>(
        &'static self,
        maybe_query: PadlResult<Q>,
    ) {
        match maybe_query {
            Ok(query) => {
                let sender = self.get_channel();
                let sender2 = self.get_channel();
                spawn_local(
                    query
                        .map_ok(move |msg| sender.send(msg).expect("Transferring data to game"))
                        .unwrap_or_else(move |e| {
                            sender2
                                .send(NetMsg::Error(e))
                                .expect("Transferring data to game")
                        }),
                );
            }
            Err(e) => {
                self.net_msg_to_game_thread(NetMsg::Error(e));
            }
        }
    }
    fn net_msg_to_game_thread(&self, msg: NetMsg) {
        let sender = self.get_channel();
        sender.send(msg).expect("Transferring data to game");
    }
}

impl std::fmt::Debug for NetMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attacks(_) => write!(f, "NetMsg: Attacks"),
            Self::Buildings(_) => write!(f, "NetMsg: Buildings"),
            Self::Error(_) => write!(f, "NetMsg: Error"),
            Self::Hobos(_, _) => write!(f, "NetMsg: Hobos"),
            Self::Leaderboard(_, _) => write!(f, "NetMsg: Leaderboard"),
            Self::Map(_, _, _) => write!(f, "NetMsg: Map"),
            Self::Player(_) => write!(f, "NetMsg: Player"),
            Self::VillageInfo(_) => write!(f, "NetMsg: VillageInfo"),
            Self::UpdateWorkerTasks(_) => write!(f, "NetMsg: UpdateWorkerTasks"),
            Self::Workers(_, _) => write!(f, "NetMsg: Workers"),
            Self::Reports(_) => write!(f, "NetMsg: Reports"),
        }
    }
}
