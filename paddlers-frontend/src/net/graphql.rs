use graphql_client::{GraphQLQuery, Response};

pub use serde::Deserialize;
type NaiveDateTime = f64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/attacks_query.graphql",
)]
pub struct AttacksQuery;
pub type AttacksResponse = Response<attacks_query::ResponseData>;


impl attacks_query::AttacksQueryVillageAttacks {
    #[allow(dead_code)]
    pub fn departure(&self) -> chrono::NaiveDateTime {
        f64_to_naive_dt(self.departure)
    }
    #[allow(dead_code)]
    pub fn arrival(&self) -> chrono::NaiveDateTime {
        f64_to_naive_dt(self.arrival)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/buildings_query.graphql",
)]
pub struct BuildingsQuery;
pub type BuildingsResponse = Response<buildings_query::ResponseData>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/resources_query.graphql",
)]
pub struct ResourcesQuery;
pub type ResourcesResponse = Response<resources_query::ResponseData>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/village_units_query.graphql",
)]
pub struct VillageUnitsQuery;
pub type VillageUnitsResponse = Response<village_units_query::ResponseData>;
pub type WorkerResponse = Vec<village_units_query::VillageUnitsQueryVillageUnits>;


fn f64_to_naive_dt(f: f64) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(f as i64, ((f%1.0) * 1_000_000.0) as u32)
}