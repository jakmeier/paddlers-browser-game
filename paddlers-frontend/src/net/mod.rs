pub mod ajax;
pub mod authentication;
pub mod game_master_api;
pub mod graphql;
pub mod state;
pub mod url;

use graphql::{
    GraphQlState,
    query_types::*,
};
use crate::game::player_info::PlayerInfo;

use stdweb::{spawn_local};

use futures::Future;
use futures::future::TryFutureExt;
use std::sync::{
    Mutex,
    mpsc::Sender,
    atomic::{AtomicBool, Ordering},
};

use crate::prelude::*;

pub enum NetMsg {
    Attacks(AttacksResponse),
    Buildings(BuildingsResponse),
    Error(PadlError),
    Map(MapResponse, i32, i32),
    Player(PlayerInfo),
    Resources(ResourcesResponse),
    UpdateWorkerTasks(WorkerTasksResponse),
    Workers(WorkerResponse),
}

pub enum NetUpdateRequest {
    CompleteReload,
    WorkerTasks(i64),
    PlayerInfo,
}

struct NetState {
    interval_ms: u32,
    logged_in: AtomicBool,
    chan: Option<Mutex<Sender<NetMsg>>>,
    gql_state: GraphQlState,
}
static mut STATIC_NET_STATE: NetState = NetState {
    interval_ms: 5_000,
    logged_in: AtomicBool::new(false),
    chan: None,
    gql_state: GraphQlState::new(),
};


/// Sets up continuous networking with the help of JS setTimeout
pub fn init_net(chan: Sender<NetMsg>) {
    unsafe{
        STATIC_NET_STATE.chan = Some(Mutex::new(chan));
    }
}
#[js_export]
/// Sets up continuous networking with the help of JS setTimeout
/// Must be called from JS once the user is logged in 
pub fn start_network_thread() {
    unsafe{
        STATIC_NET_STATE.logged_in.store(true, Ordering::Relaxed);
        // Brute-force sync up with WASM initialization
        while STATIC_NET_STATE.chan.is_none() {
            // NOP
        }
        STATIC_NET_STATE.work();
        request_client_state();
    }
}
/// Sends all requests out necessary for the client state
pub fn request_client_state() {
    unsafe {
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.buildings_query());
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.workers_query());
        request_player_update();
    }
}
pub fn request_map_read(min: i32, max: i32) {
    unsafe{
        if STATIC_NET_STATE.logged_in.load(Ordering::Relaxed) {
            STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.map_query(min, max));
        }
    }
}
pub fn request_worker_tasks_update(unit_id: i64) {
    unsafe{
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.worker_tasks_query(unit_id));
    }
}
pub fn request_player_update() {
    unsafe{
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.player_info_query());
    }
}
impl NetState {
    fn register_networking(&'static self) {
        let ms = self.interval_ms;
        stdweb::web::set_timeout(
            move || {self.work()}, 
            ms
        );
    }
    // For frequent updates
    fn work(&'static self){
        self.spawn(self.gql_state.attacks_query());
        self.spawn(self.gql_state.resource_query());
        self.spawn(self.gql_state.player_info_query());
        self.register_networking();
    }

    fn get_channel(&self) -> Sender<NetMsg> {
        match self.chan.as_ref().unwrap().lock(){
            Ok(chan) => chan.clone(),
            Err(e) => panic!("Could not get channel: {}.", e)
        }
    }

    fn spawn<Q: Future<Output = PadlResult<NetMsg>> + 'static >(&'static self, maybe_query: PadlResult<Q>) {
        match maybe_query {
            Ok(query) => {

                let sender = self.get_channel();
                let sender2 = self.get_channel();
                spawn_local(
                    query.map_ok(
                        move |msg| sender.send(msg).expect("Transferring data to game")
                    )
                    .unwrap_or_else(
                        move |e| sender2.send(NetMsg::Error(e)).expect("Transferring data to game")
                    )
                );
            },
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