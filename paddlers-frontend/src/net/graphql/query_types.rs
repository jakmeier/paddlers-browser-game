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
pub type WorkerResponse = Vec<village_units_query::VillageUnitsQueryVillageUnits>;
pub type VillageUnitsTask = village_units_query::VillageUnitsQueryVillageUnitsTasks;
pub type VillageUnitsTaskType = village_units_query::TaskType;

use crate::game::units::workers::WorkerTask;
impl Into<WorkerTask> for &VillageUnitsTask {
    fn into(self) -> WorkerTask {
        WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).0
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/unit_tasks_query.graphql",
)]
pub struct UnitTasksQuery;
pub type UnitTasksRawResponse = Response<unit_tasks_query::ResponseData>;
pub type UnitTasksResponse = unit_tasks_query::UnitTasksQueryUnit;
pub type UnitTask = unit_tasks_query::UnitTasksQueryUnitTasks;
pub type UnitTaskType = unit_tasks_query::TaskType;

impl Into<WorkerTask> for &UnitTask {
    fn into(self) -> WorkerTask {
        WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).0
        }
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
impl Into<TaskType> for &UnitTaskType {
    fn into(self) -> TaskType {
        match self {
            UnitTaskType::IDLE => TaskType::Idle,
            UnitTaskType::WALK => TaskType::Walk,
            UnitTaskType::GATHER_STICKS => TaskType::GatherSticks,
            UnitTaskType::CHOP_TREE => TaskType::ChopTree,
            UnitTaskType::DEFEND => TaskType::Defend,
            _ => panic!("Unexpected task type")
        }
    }
}
impl Into<TaskType> for UnitTaskType {
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
            _ => panic!("Unexpected task type")
        }
    }
}
impl Into<TaskType> for VillageUnitsTaskType {
    fn into(self) -> TaskType {
        (&self).into()
    }
}