mod http_calls;
pub mod query_types;
use std::future::Future;

use super::{NetMsg, NewAttackId, NewReportId};
use http_calls::*;
pub use query_types::*;

use crate::net::state::current_village_async;
use crate::prelude::*;
use paddlers_shared_lib::prelude::VillageKey;

pub struct GraphQlState {
    next_attack_id: i64,
    next_report_id: i64,
}

impl GraphQlState {
    pub(super) const fn new() -> GraphQlState {
        GraphQlState {
            next_attack_id: 0,
            next_report_id: 0,
        }
    }

    pub(super) fn update_attack_id(&mut self, id: i64) {
        self.next_attack_id = self.next_attack_id.max(id + 1);
    }
    pub(super) fn update_report_id(&mut self, id: i64) {
        self.next_report_id = self.next_report_id.max(id + 1);
    }

    pub(super) fn attacks_query(&self) -> impl Future<Output = PadlResult<NetMsg>> {
        let next = self.next_attack_id;
        async move {
            let village = current_village_async().await?;
            let response = http_read_incoming_attacks(Some(next), village).await?;
            if let Some(data) = &response.data {
                let max_id = data
                    .village
                    .attacks
                    .iter()
                    .map(|atk| atk.id.parse().unwrap())
                    .fold(0, i64::max);
                nuts::publish(NewAttackId { id: max_id });
            }
            Ok(NetMsg::Attacks(response))
        }
    }
    pub(super) async fn resource_query() -> PadlResult<NetMsg> {
        let village = current_village_async().await?;
        let response = http_read_resources(village).await?;
        Ok(NetMsg::VillageInfo(response))
    }
    pub(super) async fn buildings_query() -> PadlResult<NetMsg> {
        let village = current_village_async().await?;
        let response = http_read_buildings(village).await?;
        Ok(NetMsg::Buildings(response))
    }
    pub(super) async fn foreign_buildings_query(vid: VillageKey) -> PadlResult<NetMsg> {
        let response = http_read_buildings(vid).await?;
        Ok(NetMsg::Buildings(response))
    }
    pub(super) async fn workers_query() -> PadlResult<NetMsg> {
        let village = current_village_async().await?;
        let response = http_read_workers(village).await?;
        if let Some(data) = response.data {
            let workers = data.village.workers;
            let village = VillageKey(data.village.id);
            Ok(NetMsg::Workers(workers, village))
        } else {
            gql_empty_error("workers")
        }
    }
    pub(super) async fn hobos_query() -> PadlResult<NetMsg> {
        let village = current_village_async().await?;
        let response = http_read_hobos(village).await?;
        Ok(NetMsg::Hobos(response.hobos, VillageKey(response.id)))
    }
    pub(super) async fn foreign_hobos_query(village: VillageKey) -> PadlResult<NetMsg> {
        let response = http_read_hobos(village).await?;
        Ok(NetMsg::Hobos(response.hobos, VillageKey(response.id)))
    }
    pub(super) async fn worker_tasks_query(unit_id: i64) -> PadlResult<NetMsg> {
        let response = http_read_worker_tasks(unit_id).await?;
        if let Some(data) = response.data {
            Ok(NetMsg::UpdateWorkerTasks(data.worker))
        } else {
            gql_empty_error("worker_tasks")
        }
    }
    pub(super) async fn map_query(min: i32, max: i32) -> PadlResult<NetMsg> {
        let response = http_read_map(min as i64, max as i64).await?;
        Ok(NetMsg::Map(response, min, max))
    }

    pub async fn player_info_query() -> PadlResult<NetMsg> {
        let response = http_read_player_info().await?;
        Ok(NetMsg::Player(response.into()))
    }

    pub async fn leaderboard_query() -> PadlResult<NetMsg> {
        let response = http_read_leaderboard().await?;
        Ok(NetMsg::Leaderboard(
            1,
            response
                .into_iter()
                .map(|player| (player.display_name, player.karma))
                .collect(),
        ))
    }
    pub(super) fn reports_query(&self) -> impl Future<Output = PadlResult<NetMsg>> {
        let report_id = self.next_report_id;
        async move {
            let village = current_village_async().await?;
            let data: ReportsResponse = http_read_reports(Some(report_id), village).await?;
            let max_id = data
                .village
                .reports
                .iter()
                .map(|r| r.id.parse().unwrap())
                .fold(0, i64::max);
            nuts::publish(NewReportId { id: max_id });
            Ok(NetMsg::Reports(data))
        }
    }
}

pub async fn own_villages_query() -> PadlResult<Vec<VillageKey>> {
    let response = http_read_own_villages().await?;
    Ok(response
        .villages
        .into_iter()
        .map(|v| VillageKey(v.id as i64))
        .collect())
}

fn gql_empty_error<R>(data_set: &'static str) -> PadlResult<R> {
    PadlErrorCode::EmptyGraphQLData(data_set).dev()
}
