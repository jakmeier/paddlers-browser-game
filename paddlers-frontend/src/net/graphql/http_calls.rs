use crate::prelude::*;
use crate::net::{
    graphql::query_types::*,
    ajax,
    url::*,
};
use graphql_client::GraphQLQuery;
use futures::Future;
use futures_util::future::FutureExt;

pub (super) fn http_read_incoming_attacks(min_attack_id: Option<i64>) -> PadlResult<impl Future<Output = PadlResult<AttacksResponse>>> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables{min_attack_id: min_attack_id});
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: AttacksResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}

pub (super) fn http_read_buildings() -> PadlResult<impl Future<Output = PadlResult<BuildingsResponse>>> {
    let request_body = BuildingsQuery::build_query(buildings_query::Variables{});
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: BuildingsResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}

pub (super) fn http_read_resources() -> PadlResult<impl Future<Output = PadlResult<ResourcesResponse>>> {
    let request_body = ResourcesQuery::build_query(resources_query::Variables{});
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: ResourcesResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}

pub (super) fn http_read_workers() ->  PadlResult<impl Future<Output = PadlResult<VillageUnitsResponse>>> {
    let request_body = VillageUnitsQuery::build_query(village_units_query::Variables{});
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: VillageUnitsResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}

pub (super) fn http_read_worker_tasks(unit_id: i64) -> PadlResult<impl Future<Output = PadlResult<UnitTasksRawResponse>>> {
    let request_body = UnitTasksQuery::build_query(unit_tasks_query::Variables{ unit_id: unit_id });
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: UnitTasksRawResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}
pub (super) fn http_read_map(low_x: i64, high_x: i64) -> PadlResult<impl Future<Output = PadlResult<MapResponse>>> {
    let request_body = MapQuery::build_query(map_query::Variables{low_x, high_x});
    let request_string = &serde_json::to_string(&request_body)?;
    let promise = ajax::send("POST", &graphql_url()?, request_string)?;
    Ok(
        promise.map(|x| {
            let response: MapResponse = 
                serde_json::from_str(&x?)?;
            Ok(response)
        })
    )
}