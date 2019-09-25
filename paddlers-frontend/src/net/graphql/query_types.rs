use crate::logging::error::PadlResult;
use specs::prelude::*;
use crate::game::components::NetObj;
use paddlers_shared_lib::models::*;
use graphql_client::{GraphQLQuery, Response};
use paddlers_shared_lib::graphql_types;

pub use serde::Deserialize;
type GqlTimestamp = String;

fn timestamp(s: &String) -> graphql_types::GqlTimestamp {
    graphql_types::GqlTimestamp::from_string(s).unwrap()
}

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
pub type WorkerResponse = Vec<village_units_query::VillageUnitsQueryVillageWorkers>;
pub type VillageUnitsTask = village_units_query::VillageUnitsQueryVillageWorkersTasks;
pub type VillageUnitsTaskType = village_units_query::TaskType;
pub type VillageUnitsAbilityType = village_units_query::AbilityType;

use crate::game::units::workers::WorkerTask;
impl VillageUnitsTask {
    pub fn create(&self, net_ids: &ReadStorage<NetObj>, entities: &Entities) -> PadlResult<WorkerTask> {
        let target = 
        if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        }
        else {
            None
        };
        Ok(WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).0,
            target,
        })
    }
}
impl Into<AbilityType> for &VillageUnitsAbilityType {
    fn into(self) -> AbilityType {
        match self {
            VillageUnitsAbilityType::WORK => AbilityType::Work,
            VillageUnitsAbilityType::WELCOME => AbilityType::Welcome,
            _ => panic!("Unexpected ability")
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/worker_tasks_query.graphql",
)]
pub struct WorkerTasksQuery;
pub type WorkerTasksRawResponse = Response<worker_tasks_query::ResponseData>;
pub type WorkerTasksResponse = worker_tasks_query::WorkerTasksQueryWorker;
pub type WorkerTaskEx = worker_tasks_query::WorkerTasksQueryWorkerTasks;
pub type WorkerTaskType = worker_tasks_query::TaskType;

impl WorkerTaskEx {
    pub fn create(&self, net_ids: &ReadStorage<NetObj>, entities: &Entities) -> PadlResult<WorkerTask> {
        let target = 
        if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        }
        else {
            None
        };
        Ok(WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).0,
            target,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/map_query.graphql",
)]
pub struct MapQuery;
pub type MapResponse = Response<map_query::ResponseData>;

use paddlers_shared_lib::models::TaskType;
impl Into<TaskType> for &WorkerTaskType {
    fn into(self) -> TaskType {
        match self {
            WorkerTaskType::IDLE => TaskType::Idle,
            WorkerTaskType::WALK => TaskType::Walk,
            WorkerTaskType::GATHER_STICKS => TaskType::GatherSticks,
            WorkerTaskType::CHOP_TREE => TaskType::ChopTree,
            WorkerTaskType::DEFEND => TaskType::Defend,
            WorkerTaskType::WELCOME_ABILITY => TaskType::WelcomeAbility,
            _ => panic!("Unexpected task type")
        }
    }
}
impl Into<TaskType> for WorkerTaskType {
    fn into(self) -> TaskType {
        (&self).into()
    }
}
impl Into<TaskType> for &VillageUnitsTaskType {
    fn into(self) -> TaskType {
        match self {
            VillageUnitsTaskType::IDLE => TaskType::Idle,
            VillageUnitsTaskType::WALK => TaskType::Walk,
            VillageUnitsTaskType::GATHER_STICKS => TaskType::GatherSticks,
            VillageUnitsTaskType::CHOP_TREE => TaskType::ChopTree,
            VillageUnitsTaskType::DEFEND => TaskType::Defend,
            VillageUnitsTaskType::WELCOME_ABILITY => TaskType::WelcomeAbility,
            _ => panic!("Unexpected task type")
        }
    }
}
impl Into<TaskType> for VillageUnitsTaskType {
    fn into(self) -> TaskType {
        (&self).into()
    }
}