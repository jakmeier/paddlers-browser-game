pub mod ajax;
pub mod authentication;
pub mod game_master_api;
pub mod graphql;
pub mod state;
pub mod url;

use crate::{game::player_info::PlayerInfo, web_integration::*};
use game_master_api::RestApiState;
use graphql::{query_types::*, GraphQlState};
use paddle::Domain;
use paddlers_shared_lib::prelude::VillageKey;
use wasm_bindgen::prelude::*;

use futures::future::TryFutureExt;
use futures::Future;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
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
    chan: Sender<NetMsg>,
    logged_in: AtomicBool,
    gql_state: GraphQlState,
}

const NET_THREAD_TIMEOUT_MS: i32 = 5000;

// Requests
struct NetworkUpdate;
struct RequestPlayerUpdate;
struct RequestClientStateUpdate;
struct RequestResourceUpdate;
struct LoggedIn;
struct RequestMapRead {
    min: i32,
    max: i32,
}
struct RequestWorkerTasksUpdate {
    unit_id: i64,
}
struct RequestForeignTownUpdate {
    vid: VillageKey,
}

// Update for responses
struct NewAttackId {
    id: i64,
}
struct NewReportId {
    id: i64,
}

/// Initializes state necessary for networking
pub fn init_net(chan: Sender<NetMsg>) {
    NetState::init(chan);
    RestApiState::init();
}

#[wasm_bindgen]
/// Sets up continuous networking with the help of JS setTimeout
/// Must be called from JS once the user is logged in
pub fn start_network_thread() {
    nuts::publish(LoggedIn);
}
/// Sends all requests out necessary for the client state to display a full game view including the home town
pub fn request_client_state() {
    nuts::publish(RequestClientStateUpdate);
}
pub fn request_map_read(min: i32, max: i32) {
    nuts::publish(RequestMapRead { min, max });
}
pub fn request_worker_tasks_update(unit_id: i64) {
    nuts::publish(RequestWorkerTasksUpdate { unit_id });
}
pub fn request_resource_update() {
    nuts::publish(RequestResourceUpdate);
}
pub fn request_player_update() {
    nuts::publish(RequestPlayerUpdate);
}
pub fn request_foreign_town(vid: VillageKey) {
    nuts::publish(RequestForeignTownUpdate { vid });
}

impl NetState {
    fn init(chan: Sender<NetMsg>) {
        let ns = NetState {
            logged_in: AtomicBool::new(false),
            chan,
            gql_state: GraphQlState::new(),
        };
        let net_activity = nuts::new_domained_activity(ns, &Domain::Network);
        net_activity.subscribe(NetState::log_in);
        net_activity.subscribe(NetState::work);
        net_activity.subscribe(NetState::request_player_update);
        net_activity.subscribe(NetState::request_client_state);
        net_activity.subscribe(NetState::request_resource_update);
        net_activity.subscribe(NetState::request_map_read);
        net_activity.subscribe(NetState::request_worker_tasks_update);
        net_activity.subscribe(NetState::request_foreign_town);
        net_activity.subscribe(NetState::update_attack_id);
        net_activity.subscribe(NetState::update_report_id);
    }

    // For frequent updates
    fn work(&mut self, _: &NetworkUpdate) {
        self.spawn(self.gql_state.attacks_query());
        self.spawn(self.gql_state.reports_query());
        self.spawn(self.gql_state.resource_query());
        self.spawn(GraphQlState::player_info_query());
    }

    // Sends all requests out necessary for the client state to display a full game view including the home town
    fn request_client_state(&mut self, _: &RequestClientStateUpdate) {
        // TODO: Instead of forcing a request every time the 10s are too long, use something smarter.
        // E.g.: Once a second check what needs to be updated and then allow this list to be altered from outside
        if self.logged_in.load(Ordering::Relaxed) {
            self.spawn(self.gql_state.buildings_query());
            self.spawn(self.gql_state.workers_query());
            self.spawn(self.gql_state.hobos_query());
            self.spawn(self.gql_state.leaderboard_query());
            self.spawn(self.gql_state.attacks_query());
            self.spawn(self.gql_state.resource_query());
            request_player_update();
        } else {
            let mut thread = crate::web_integration::create_thread(request_client_state);
            thread.set_timeout(50);
            nuts::store_to_domain(&Domain::Network, thread);
        }
    }

    fn request_foreign_town(&mut self, msg: &RequestForeignTownUpdate) {
        self.spawn(Ok(self.gql_state.foreign_buildings_query(msg.vid)));
        self.spawn(Ok(self.gql_state.foreign_hobos_query(msg.vid)));
        // TODO: Other state
    }

    fn request_player_update(&mut self, _: &RequestPlayerUpdate) {
        if self.logged_in.load(Ordering::Relaxed) {
            self.spawn(GraphQlState::player_info_query());
        } else {
            let mut thread = crate::web_integration::create_thread(request_player_update);
            thread.set_timeout(50);
            nuts::store_to_domain(&Domain::Network, (thread,));
        }
    }

    fn request_worker_tasks_update(&mut self, msg: &RequestWorkerTasksUpdate) {
        self.spawn(self.gql_state.worker_tasks_query(msg.unit_id));
    }

    fn request_resource_update(&mut self, _: &RequestResourceUpdate) {
        self.spawn(self.gql_state.resource_query());
    }

    fn request_map_read(&mut self, msg: &RequestMapRead) {
        self.spawn(self.gql_state.map_query(msg.min, msg.max));
    }

    fn log_in(&mut self, _: &LoggedIn) {
        self.logged_in.store(true, Ordering::Relaxed);
        let work_thread = start_thread(|| nuts::publish(NetworkUpdate), NET_THREAD_TIMEOUT_MS);
        nuts::store_to_domain(&Domain::Network, work_thread)
    }

    fn update_attack_id(&mut self, msg: &NewAttackId) {
        self.gql_state.update_attack_id(msg.id);
    }
    fn update_report_id(&mut self, msg: &NewReportId) {
        self.gql_state.update_report_id(msg.id);
    }

    fn get_channel(&self) -> Sender<NetMsg> {
        self.chan.clone()
    }

    fn spawn<Q: Future<Output = PadlResult<NetMsg>> + 'static>(&self, maybe_query: PadlResult<Q>) {
        match maybe_query {
            Ok(query) => {
                let sender = self.get_channel();
                let sender2 = self.get_channel();
                wasm_bindgen_futures::spawn_local(
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
