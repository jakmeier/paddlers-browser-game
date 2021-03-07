use crate::game::components::NetObj;
use crate::game::units::workers::WorkerTask;
use crate::prelude::*;
use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use paddlers_shared_lib::graphql_types;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::story::story_state::StoryState;
use specs::prelude::*;

pub use serde::Deserialize;
type GqlTimestamp = String;

pub fn parse_timestamp(s: &String) -> NaiveDateTime {
    timestamp(s).to_chrono()
}
fn timestamp(s: &String) -> graphql_types::GqlTimestamp {
    graphql_types::GqlTimestamp::from_string(s).unwrap()
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/attacks_query.graphql",
    response_derives = "PartialEq",
    extern_enums("UnitColor", "HoboAttributeType")
)]
pub struct AttacksQuery;
pub type AttacksResponse = attacks_query::ResponseData;
pub type HoboEffect = attacks_query::AttacksQueryVillageAttacksUnitsHoboEffects;

impl attacks_query::AttacksQueryVillageAttacks {
    #[allow(dead_code)]
    pub fn departure(&self) -> chrono::NaiveDateTime {
        timestamp(&self.departure).to_chrono()
    }
    #[allow(dead_code)]
    pub fn arrival(&self) -> chrono::NaiveDateTime {
        timestamp(&self.arrival).to_chrono()
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/buildings_query.graphql",
    extern_enums("BuildingType")
)]
pub struct BuildingsQuery;
pub type BuildingsResponse = buildings_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/volatile_village_info_query.graphql"
)]
pub struct VolatileVillageInfoQuery;
pub type VolatileVillageInfoResponse = volatile_village_info_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/village_units_query.graphql",
    extern_enums("TaskType", "AbilityType")
)]
pub struct VillageUnitsQuery;
pub type VillageUnitsResponse = village_units_query::ResponseData;
pub type WorkerResponse = Vec<village_units_query::VillageUnitsQueryVillageWorkers>;
pub type VillageUnitsTask = village_units_query::VillageUnitsQueryVillageWorkersTasks;

impl VillageUnitsTask {
    pub fn create(
        &self,
        net_ids: &ReadStorage<NetObj>,
        entities: &Entities,
    ) -> PadlResult<WorkerTask> {
        let target = if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        } else {
            None
        };
        Ok(WorkerTask {
            task_type: self.task_type,
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).to_chrono(),
            target,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/worker_tasks_query.graphql",
    extern_enums("TaskType")
)]
pub struct WorkerTasksQuery;
pub type WorkerTasksRawResponse = worker_tasks_query::ResponseData;
pub type WorkerTasksResponse = worker_tasks_query::WorkerTasksQueryWorker;
pub type WorkerTaskEx = worker_tasks_query::WorkerTasksQueryWorkerTasks;

impl WorkerTaskEx {
    pub fn create(
        &self,
        net_ids: &ReadStorage<NetObj>,
        entities: &Entities,
    ) -> PadlResult<WorkerTask> {
        let target = if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        } else {
            None
        };
        Ok(WorkerTask {
            task_type: self.task_type,
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).to_chrono(),
            target,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/hobos_query.graphql",
    response_derives = "PartialEq"
)]
pub struct HobosQuery;
pub type HobosQueryRawResponse = hobos_query::ResponseData;
pub type HobosQueryResponseVillage = hobos_query::HobosQueryVillage;
pub type HobosQueryResponse = Vec<hobos_query::HobosQueryVillageHobos>;
pub type HobosQueryUnitColor = hobos_query::UnitColor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_query.graphql",
    extern_enums("StoryState")
)]
pub struct PlayerQuery;
pub type PlayerQueryRawResponse = player_query::ResponseData;
pub type PlayerQueryResponse = player_query::PlayerQueryPlayer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/map_query.graphql"
)]
pub struct MapQuery;
pub type MapResponse = map_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_villages_query.graphql"
)]
pub struct PlayerVillagesQuery;
pub type PlayerVillagesRawResponse = player_villages_query::ResponseData;
pub type PlayerVillagesResponse = player_villages_query::PlayerVillagesQueryPlayer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_quests_query.graphql",
    extern_enums("BuildingType", "TaskType")
)]
pub struct PlayerQuestsQuery;
pub type QuestsRawResponse = player_quests_query::ResponseData;
pub type QuestsResponse = Vec<player_quests_query::PlayerQuestsQueryPlayerQuests>;
pub type PlayerQuest = player_quests_query::PlayerQuestsQueryPlayerQuests;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/leaderboard_query.graphql"
)]
pub struct LeaderboardQuery;
pub type LeaderboardRawResponse = leaderboard_query::ResponseData;
pub type LeaderboardResponse = Vec<leaderboard_query::LeaderboardQueryScoreboard>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/reports_query.graphql",
    extern_enums("UnitColor")
)]
pub struct ReportsQuery;
pub type ReportsResponse = reports_query::ResponseData;
pub type ReportsResponseReport = reports_query::ReportsQueryVillageReports;
