use crate::net::{
    GRAPH_QL_PATH,
    graphql::query_types::*,
    ajax,
};
use graphql_client::GraphQLQuery;
use futures::Future;
use futures_util::future::FutureExt;

pub (super) fn http_read_incoming_attacks(min_attack_id: Option<i64>) -> impl Future<Output = AttacksResponse> {
    let request_body = AttacksQuery::build_query(attacks_query::Variables{min_attack_id: min_attack_id});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: AttacksResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub (super) fn http_read_buildings() -> impl Future<Output = BuildingsResponse> {
    let request_body = BuildingsQuery::build_query(buildings_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: BuildingsResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub (super) fn http_read_resources() -> impl Future<Output = ResourcesResponse> {
    let request_body = ResourcesQuery::build_query(resources_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: ResourcesResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub (super) fn http_read_workers() -> impl Future<Output = VillageUnitsResponse> {
    let request_body = VillageUnitsQuery::build_query(village_units_query::Variables{});
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: VillageUnitsResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}

pub (super) fn http_read_worker_tasks(unit_id: i64) -> impl Future<Output = UnitTasksRawResponse> {
    let request_body = UnitTasksQuery::build_query(unit_tasks_query::Variables{ unit_id: unit_id });
    let request_string = &serde_json::to_string(&request_body).unwrap();
    let promise = ajax::send("POST", GRAPH_QL_PATH, request_string);
    promise.map(|x| {
        let response: UnitTasksRawResponse = 
            serde_json::from_str(&x.unwrap()).unwrap();
        response
    })
}