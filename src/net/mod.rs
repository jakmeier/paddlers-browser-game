pub mod graphql;
pub mod ajax;

use graphql::*;

use graphql_client::{GraphQLQuery};

use stdweb::{spawn_local};

use futures::Future;
use futures_util::future::FutureExt;
use std::sync::{
    Mutex,
    mpsc::Sender,
};

const GRAPH_QL_PATH: &'static str = "http://localhost:65432/graphql";
const SHOP_PATH: &'static str = "http://localhost:8088/shop";

struct NetState {
    interval_ms: u32,
    chan: Option<Mutex<Sender<NetMsg>>>,
}
static mut STATIC_NET_STATE: NetState = NetState {
    interval_ms: 5_000,
    chan: None,
};

pub enum NetMsg {
    Attacks(AttacksResponse),
    Buildings(BuildingsResponse),
}

/// Sets up continuous networking with the help of JS setTimeout
pub fn init_net(chan: Sender<NetMsg>) {
    unsafe{
        STATIC_NET_STATE.chan = Some(Mutex::new(chan));
        STATIC_NET_STATE.work();
        let buildings_fp = http_read_buildings();

        // requests done only once
        let sender = STATIC_NET_STATE.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            buildings_fp.map(
                move |response|
                sender.send(NetMsg::Buildings(response)).expect("Transferring buildings data to game")
            )
        );
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
    // TODO: Eventually, networking should be smarter and use some kind of revision ids together with the requests
    fn work(&'static self){

        let fp = http_read_incoming_attacks();

        let sender = self.chan.as_ref().unwrap().lock().unwrap().clone();
        spawn_local(
            fp.map(
                move |response|
                sender.send(NetMsg::Attacks(response)).expect("Transferring data to game")
            )
        );

        self.register_networking();
    }
}

pub fn http_read_incoming_attacks() -> impl Future<Output = AttacksResponse> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables{});
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