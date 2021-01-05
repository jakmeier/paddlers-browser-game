use crate::net::{ajax, graphql::query_types::*, url::*};
use crate::prelude::*;
use graphql_client::GraphQLQuery;
use paddlers_shared_lib::prelude::*;

pub(super) async fn http_read_incoming_attacks(
    min_attack_id: Option<i64>,
    village_id: VillageKey,
) -> PadlResult<attacks_query::ResponseData> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables {
        min_attack_id,
        village_id: village_id.num(),
    });
    ajax::gql_query(&graphql_url()?, &request_body).await
}

pub(super) async fn http_read_buildings(
    village_id: VillageKey,
) -> PadlResult<buildings_query::ResponseData> {
    let request_body = BuildingsQuery::build_query(buildings_query::Variables {
        village_id: village_id.num(),
    });
    ajax::gql_query(&graphql_url()?, &request_body).await
}

pub(super) async fn http_read_resources(
    village_id: VillageKey,
) -> PadlResult<volatile_village_info_query::ResponseData> {
    let request_body =
        VolatileVillageInfoQuery::build_query(volatile_village_info_query::Variables {
            village_id: village_id.num(),
        });
    ajax::gql_query(&graphql_url()?, &request_body).await
}

pub(super) async fn http_read_workers(village_id: VillageKey) -> PadlResult<VillageUnitsResponse> {
    let request_body = VillageUnitsQuery::build_query(village_units_query::Variables {
        village_id: village_id.num(),
    });
    ajax::gql_query(&graphql_url()?, &request_body).await
}

pub(super) async fn http_read_hobos(
    village_id: VillageKey,
) -> PadlResult<HobosQueryResponseVillage> {
    let request_body = HobosQuery::build_query(hobos_query::Variables {
        village_id: village_id.num(),
    });
    let response: HobosQueryRawResponse = ajax::gql_query(&graphql_url()?, &request_body).await?;
    Ok(response.village)
}

pub(super) async fn http_read_worker_tasks(unit_id: i64) -> PadlResult<WorkerTasksRawResponse> {
    let request_body =
        WorkerTasksQuery::build_query(worker_tasks_query::Variables { worker_id: unit_id });
    ajax::gql_query(&graphql_url()?, &request_body).await
}
pub(super) async fn http_read_map(low_x: i64, high_x: i64) -> PadlResult<MapResponse> {
    let request_body = MapQuery::build_query(map_query::Variables { low_x, high_x });
    ajax::gql_query(&graphql_url()?, &request_body).await
}
pub(super) async fn http_read_own_villages() -> PadlResult<PlayerVillagesResponse> {
    let request_body = PlayerVillagesQuery::build_query(player_villages_query::Variables);
    let raw_response: PlayerVillagesRawResponse =
        ajax::gql_query(&graphql_url()?, &request_body).await?;
    let response = raw_response.player;
    Ok(response)
}
pub(super) async fn http_read_player_info() -> PadlResult<PlayerQueryResponse> {
    let request_body = PlayerQuery::build_query(player_query::Variables);
    let raw_response: PlayerQueryRawResponse =
        ajax::gql_query(&graphql_url()?, &request_body).await?;
    let response = raw_response.player;
    Ok(response)
}
pub(super) async fn http_read_quests() -> PadlResult<QuestsResponse> {
    let request_body = PlayerQuestsQuery::build_query(player_quests_query::Variables);
    let raw_response: QuestsRawResponse =
        ajax::gql_query(&graphql_url()?, &request_body).await?;
    let response = raw_response.player;
    Ok(response)
}

pub(super) async fn http_read_leaderboard() -> PadlResult<LeaderboardResponse> {
    let request_body = LeaderboardQuery::build_query(leaderboard_query::Variables { offset: 0 });
    let raw_response: LeaderboardRawResponse =
        ajax::gql_query(&graphql_url()?, &request_body).await?;
    let response = raw_response.scoreboard;
    Ok(response)
}

pub(super) async fn http_read_reports(
    min_report_id: Option<i64>,
    village_id: VillageKey,
) -> PadlResult<ReportsResponse> {
    let request_body = ReportsQuery::build_query(reports_query::Variables {
        min_report_id,
        village_id: village_id.num(),
    });
    ajax::gql_query(&graphql_url()?, &request_body).await
}
