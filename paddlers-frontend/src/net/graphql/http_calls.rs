use crate::net::{ajax, graphql::query_types::*, url::*};
use crate::prelude::*;
use futures::Future;
use futures_util::future::FutureExt;
use graphql_client::GraphQLQuery;
use paddlers_shared_lib::prelude::*;

pub(super) fn http_read_incoming_attacks(
    min_attack_id: Option<i64>,
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<AttacksResponse>>> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables {
        min_attack_id,
        village_id: village_id.num(),
    });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: AttacksResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}

pub(super) fn http_read_buildings(
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<BuildingsResponse>>> {
    let request_body = BuildingsQuery::build_query(buildings_query::Variables {
        village_id: village_id.num(),
    });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: BuildingsResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}

pub(super) fn http_read_resources(
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<VolatileVillageInfoResponse>>> {
    let request_body =
        VolatileVillageInfoQuery::build_query(volatile_village_info_query::Variables {
            village_id: village_id.num(),
        });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: VolatileVillageInfoResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}

pub(super) fn http_read_workers(
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<VillageUnitsResponse>>> {
    let request_body = VillageUnitsQuery::build_query(village_units_query::Variables {
        village_id: village_id.num(),
    });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: VillageUnitsResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}

pub(super) fn http_read_hobos(
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<HobosQueryResponseVillage>>> {
    let request_body = HobosQuery::build_query(hobos_query::Variables {
        village_id: village_id.num(),
    });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let raw_response: HobosQueryRawResponse = serde_json::from_str(&x?)?;
        let data =
            raw_response
                .data
                .ok_or(PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                    "village hobos",
                )))?;
        Ok(data.village)
    }))
}

pub(super) fn http_read_worker_tasks(
    unit_id: i64,
) -> PadlResult<impl Future<Output = PadlResult<WorkerTasksRawResponse>>> {
    let request_body =
        WorkerTasksQuery::build_query(worker_tasks_query::Variables { worker_id: unit_id });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: WorkerTasksRawResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}
pub(super) fn http_read_map(
    low_x: i64,
    high_x: i64,
) -> PadlResult<impl Future<Output = PadlResult<MapResponse>>> {
    let request_body = MapQuery::build_query(map_query::Variables { low_x, high_x });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let response: MapResponse = serde_json::from_str(&x?)?;
        Ok(response)
    }))
}
pub(super) fn http_read_own_villages(
) -> PadlResult<impl Future<Output = PadlResult<PlayerVillagesResponse>>> {
    let request_body = PlayerVillagesQuery::build_query(player_villages_query::Variables);
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let raw_response: PlayerVillagesRawResponse = serde_json::from_str(&x?)?;
        let response =
            raw_response
                .data
                .ok_or(PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                    "own villages",
                )))?;
        let response = response.player;
        Ok(response)
    }))
}
pub(super) fn http_read_player_info(
) -> PadlResult<impl Future<Output = PadlResult<PlayerQueryResponse>>> {
    let request_body = PlayerQuery::build_query(player_query::Variables);
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let raw_response: PlayerQueryRawResponse = serde_json::from_str(&x?)?;
        let response =
            raw_response
                .data
                .ok_or(PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                    "player info",
                )))?;
        let response = response.player;
        Ok(response)
    }))
}

pub(super) fn http_read_leaderboard(
) -> PadlResult<impl Future<Output = PadlResult<LeaderboardResponse>>> {
    let request_body = LeaderboardQuery::build_query(leaderboard_query::Variables { offset: 0 });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let raw_response: LeaderboardRawResponse = serde_json::from_str(&x?)?;
        let response =
            raw_response
                .data
                .ok_or(PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                    "leaderboard",
                )))?;
        let response = response.scoreboard;
        Ok(response)
    }))
}

pub(super) fn http_read_reports(
    min_report_id: Option<i64>,
    village_id: VillageKey,
) -> PadlResult<impl Future<Output = PadlResult<ReportsResponse>>> {
    let request_body = ReportsQuery::build_query(reports_query::Variables {
        min_report_id,
        village_id: village_id.num(),
    });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(promise.map(|x| {
        let raw_response: ReportsRawResponse = serde_json::from_str(&x?)?;
        let response =
            raw_response
                .data
                .ok_or(PadlError::dev_err(PadlErrorCode::InvalidGraphQLData(
                    "reports",
                )))?;
        Ok(response)
    }))
}
