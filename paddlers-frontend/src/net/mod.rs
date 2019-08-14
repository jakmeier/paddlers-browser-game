pub mod graphql;
pub mod ajax;
pub mod game_master_api;

use graphql::{
    GraphQlState,
    query_types::*,
};

use stdweb::{spawn_local};

use futures::Future;
use futures::future::TryFutureExt;
use std::sync::{
    Mutex,
    mpsc::Sender,
};

use crate::prelude::*;

// TODO: Better constant handling: How to read uri at compile time (from TOML file)?
// const GRAPH_QL_PATH: &'static str = "http://192.168.1.115:65432/graphql";
// const SHOP_PATH: &'static str = "http://192.168.1.115:8088/shop";
// const WORKER_PATH: &'static str = "http://192.168.1.115:8088/worker";

const GRAPH_QL_PATH: &'static str = "http://localhost:65432/graphql";
const SHOP_PATH: &'static str = "http://localhost:8088/shop";
const WORKER_PATH: &'static str = "http://localhost:8088/worker";

pub enum NetMsg {
    Attacks(AttacksResponse),
    Buildings(BuildingsResponse),
    Error(PadlError),
    Resources(ResourcesResponse),
    UpdateWorkerTasks(UnitTasksResponse),
    Workers(WorkerResponse),
}

pub enum NetUpdateRequest {
    UnitTasks(i64),
}

struct NetState {
    interval_ms: u32,
    chan: Option<Mutex<Sender<NetMsg>>>,
    gql_state: GraphQlState,
}
static mut STATIC_NET_STATE: NetState = NetState {
    interval_ms: 5_000,
    chan: None,
    gql_state: GraphQlState::new(),
};


/// Sets up continuous networking with the help of JS setTimeout
pub fn init_net(chan: Sender<NetMsg>) {
    unsafe{
        STATIC_NET_STATE.chan = Some(Mutex::new(chan));
        STATIC_NET_STATE.work();

        // requests done only once
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.buildings_query());
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.workers_query());
    }
}
pub fn request_unit_tasks_update(unit_id: i64) {
    unsafe{
        STATIC_NET_STATE.spawn(STATIC_NET_STATE.gql_state.worker_tasks_query(unit_id));
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