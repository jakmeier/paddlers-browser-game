pub mod graphql;
pub mod ajax;
pub mod game_master_api;

use graphql::*;

use graphql_client::{GraphQLQuery};

use stdweb::{spawn_local};

use futures::Future;
use futures_util::future::FutureExt;
use std::sync::{
    Mutex,
    mpsc::Sender,
};
use std::sync::atomic::{AtomicI64, Ordering};

const GRAPH_QL_PATH: &'static str = "http://localhost:65432/graphql";
const SHOP_PATH: &'static str = "http://localhost:8088/shop";

struct NetState {
    interval_ms: u32,
    chan: Option<Mutex<Sender<NetMsg>>>,
    next_attack_id: AtomicI64,
}
static mut STATIC_NET_STATE: NetState = NetState {
    interval_ms: 5_000,
    chan: None,
    next_attack_id: AtomicI64::new(0),
};

pub enum NetMsg {
    Attacks(AttacksResponse),
    Buildings(BuildingsResponse),
    Resources(ResourcesResponse),
    Workers(WorkerResponse)
}

/// Sets up continuous networking with the help of JS setTimeout
pub fn init_net(chan: Sender<NetMsg>) {
    unsafe{
        STATIC_NET_STATE.chan = Some(Mutex::new(chan));
        STATIC_NET_STATE.work();

        // requests done only once
        STATIC_NET_STATE.spawn_buildings_query();
        STATIC_NET_STATE.spawn_workers_query();
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
    fn work(&'static self){
        self.spawn_attacks_query();
        self.spawn_resource_query();
        self.register_networking();
    }

    fn spawn_attacks_query(&'static self) {
        let fp = http_read_incoming_attacks(Some(self.next_attack_id.load(Ordering::Relaxed)));
        let sender = self.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            fp.map(
                move |response| {
                    if let Some(data) = &response.data {
                        let max_id = data.village.attacks.iter()
                            .map(|atk| atk.id.parse().unwrap())
                            .fold(0, i64::max);
                        let next = self.next_attack_id.load(Ordering::Relaxed).max(max_id + 1);
                        self.next_attack_id.store(next, Ordering::Relaxed);
                    }
                    sender.send(NetMsg::Attacks(response)).expect("Transferring data to game")
                }
            )
        );        
    }
    fn spawn_resource_query(&self) {
        let res_fp = http_read_resources();
        let sender = self.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            res_fp.map(
                move |response|
                sender.send(NetMsg::Resources(response)).expect("Transferring resource data to game")
            )
        );
    }
    fn spawn_buildings_query(&self) {
        let buildings_fp = http_read_buildings();
        let sender = self.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            buildings_fp.map(
                move |response|
                sender.send(NetMsg::Buildings(response)).expect("Transferring buildings data to game")
            )
        );
    }
    fn spawn_workers_query(&self) {
        let fp = http_read_workers();
        let sender = self.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            fp.map(
                move |response| {
                    let workers = response.data.unwrap().village.units;
                    sender.send(NetMsg::Workers(workers)).expect("Transferring data to game")
                }
            )
        );
    }
}

pub fn http_read_incoming_attacks(min_attack_id: Option<i64>) -> impl Future<Output = AttacksResponse> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables{min_attack_id: min_attack_id});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: AttacksResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub fn http_read_buildings() -> impl Future<Output = BuildingsResponse> {
    let request_body = BuildingsQuery::build_query(buildings_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: BuildingsResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub fn http_read_resources() -> impl Future<Output = ResourcesResponse> {
    let request_body = ResourcesQuery::build_query(resources_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: ResourcesResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub fn http_read_workers() -> impl Future<Output = VillageUnitsResponse> {
    let request_body = VillageUnitsQuery::build_query(village_units_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: VillageUnitsResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}