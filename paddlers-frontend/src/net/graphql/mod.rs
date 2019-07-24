pub mod query_types;
mod http_calls;
pub use query_types::*;
use http_calls::*;
use super::NetMsg;

use futures::Future;
use futures_util::future::FutureExt;
use std::sync::atomic::{AtomicI64, Ordering};

pub struct GraphQlState {
    next_attack_id: AtomicI64,
}


impl GraphQlState {

    pub (super) const fn new() -> GraphQlState {
        GraphQlState {
            next_attack_id: AtomicI64::new(0),
        }
    }

    pub (super) fn attacks_query(&'static self) -> impl Future<Output = NetMsg> {
        let fp = http_read_incoming_attacks(Some(self.next_attack_id.load(Ordering::Relaxed)));
        fp.map(
            move |response| {
                if let Some(data) = &response.data {
                    let max_id = data.village.attacks.iter()
                        .map(|atk| atk.id.parse().unwrap())
                        .fold(0, i64::max);
                    let next = self.next_attack_id.load(Ordering::Relaxed).max(max_id + 1);
                    self.next_attack_id.store(next, Ordering::Relaxed);
                }
                NetMsg::Attacks(response)
            }
        )        
    }
    pub (super) fn resource_query(&self) -> impl Future<Output = NetMsg> {
        let res_fp = http_read_resources();
        res_fp.map(
            |response|
            NetMsg::Resources(response)
        )
    }
    pub (super) fn buildings_query(&self) -> impl Future<Output = NetMsg> {
        let buildings_fp = http_read_buildings();
        buildings_fp.map(
            |response|
            NetMsg::Buildings(response)
        )
    }
    pub (super) fn workers_query(&self) -> impl Future<Output = NetMsg> {
        let fp = http_read_workers();
        fp.map(
            |response| {
                if let Some(data) = response.data {
                    let workers = data.village.units;
                    NetMsg::Workers(workers)
                }
                else {
                    NetMsg::Error("Received empty response".to_owned())
                }
            }
        )
    }
    pub (super) fn worker_tasks_query(&self, unit_id: i64) -> impl Future<Output = NetMsg> {
        let fp = http_read_worker_tasks(unit_id);
        fp.map(
            |response| {
                if let Some(data) = response.data {
                    NetMsg::UpdateWorkerTasks(data.unit)
                }
                else {
                    NetMsg::Error("Received empty response".to_owned())
                }
            }
        )
    }
}