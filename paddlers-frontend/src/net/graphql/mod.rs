pub mod query_types;
mod http_calls;
pub use query_types::*;
use http_calls::*;
use super::NetMsg;

use futures::Future;
use futures::future::TryFuture;
use futures_util::future::FutureExt;
use futures_util::try_future::TryFutureExt;
use std::sync::atomic::{AtomicI64, Ordering};
use crate::prelude::*;
use crate::net::state::current_village_async;
use paddlers_shared_lib::prelude::VillageKey;

pub struct GraphQlState {
    next_attack_id: AtomicI64,
}

impl GraphQlState {

    pub (super) const fn new() -> GraphQlState {
        GraphQlState {
            next_attack_id: AtomicI64::new(0),
        }
    }

    pub (super) fn attacks_query(&'static self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        current_village_async()
            .map(|fut|
                fut.and_then(move |village: VillageKey|
                    http_read_incoming_attacks(Some(self.next_attack_id.load(Ordering::Relaxed)), village)
                        .expect("Query building error")
                )
                .map(
                    move |response| {
                        let r : AttacksResponse = response?;
                        if let Some(data) = &r.data {
                            let max_id = data.village.attacks.iter()
                                .map(|atk| atk.id.parse().unwrap())
                                .fold(0, i64::max);
                            let next = self.next_attack_id.load(Ordering::Relaxed).max(max_id + 1);
                            self.next_attack_id.store(next, Ordering::Relaxed);
                        }
                        Ok(NetMsg::Attacks(r))
                    }
                )
            )
    }
    pub (super) fn resource_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        current_village_async()
            .map(|fut|
                fut.and_then(move |village: VillageKey|
                    http_read_resources(village)
                        .expect("Query building error")
                ).map(
                    |response|
                    Ok(NetMsg::Resources(response?))
                )
            )
    }
    pub (super) fn buildings_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        current_village_async()
            .map(|fut|
                fut.and_then(move |village: VillageKey|
                    http_read_buildings(village)
                        .expect("Query building error")
                ).map(
                    |response|
                    Ok(NetMsg::Buildings(response?))
                )
            )
    }
    pub (super) fn workers_query(&self) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        current_village_async()
            .map(|fut|
                fut.and_then(move |village: VillageKey| 
                    http_read_workers(village)
                        .expect("Query building error")
                ).map(
                    |response| {
                        if let Some(data) = response?.data {
                            let workers = data.village.workers;
                            Ok(NetMsg::Workers(workers))
                        }
                        else {
                            gql_empty_error("workers")
                        }
                    }
                )
            )
    }
    pub (super) fn worker_tasks_query(&self, unit_id: i64) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_worker_tasks(unit_id)?;
        Ok(
            fp.map(
                |response| {
                    if let Some(data) = response?.data {
                        Ok(NetMsg::UpdateWorkerTasks(data.worker))
                    }
                    else {
                        gql_empty_error("worker_tasks")
                    }
                }
            )
        )
    }
    pub (super) fn map_query(&self, min: i32, max: i32) -> PadlResult<impl Future<Output = PadlResult<NetMsg>>> {
        let fp = http_read_map(min as i64, max as i64)?;
        Ok(
            fp.map(
                move |response| Ok(NetMsg::Map(response?, min, max)),
            )
        )
    }
}

pub fn own_villages_query() -> PadlResult<impl TryFuture<Ok = Vec<VillageKey>, Error = PadlError>> {
    let fp = http_read_own_villages()?;
    Ok(
        fp.map(
            move |response| Ok(response?.villages.into_iter().map(|v|VillageKey(v.id as i64)).collect()),
        )
    )
}

fn gql_empty_error<R>(data_set: &'static str) -> PadlResult<R> {
    PadlErrorCode::EmptyGraphQLData(data_set).dev()
}