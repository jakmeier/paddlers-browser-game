//! Some helper types to represent data more concise.
//! Types defined in here should not access the database, to preserve the conditions stated in gql_public.

use super::*;
use juniper;
use paddlers_shared_lib::prelude::ResourceType;

pub struct Resources {
    res: Vec<(ResourceType, i64)>,
}

pub struct BuildingCondition {
    bt: BuildingType,
    amount: i32,
}
pub struct WorkerCondition {
    tt: TaskType,
    amount: i32,
}

pub struct QuestConditions {
    res: Resources,
    karma: Option<i32>,
    pop: Option<i32>,
    buildings: Vec<BuildingCondition>,
    worker: Vec<WorkerCondition>,
}

#[juniper::object (Context = Context)]
impl Resources {
    pub fn feathers(&self) -> i32 {
        self.resource(ResourceType::Feathers)
    }
    pub fn sticks(&self) -> i32 {
        self.resource(ResourceType::Sticks)
    }
    pub fn logs(&self) -> i32 {
        self.resource(ResourceType::Logs)
    }
}

#[juniper::object (Context = Context)]
impl QuestConditions {
    pub fn karma(&self) -> Option<i32> {
        self.karma
    }
    pub fn pop(&self) -> Option<i32> {
        self.pop
    }
    pub fn resources(&self) -> &Resources {
        &self.res
    }
    pub fn buildings(&self) -> &[BuildingCondition] {
        &self.buildings
    }
    pub fn workers(&self) -> &[WorkerCondition] {
        &self.worker
    }
}

#[juniper::object (Context = Context)]
impl BuildingCondition {
    pub fn building_type(&self) -> BuildingType {
        self.bt
    }
    pub fn amount(&self) -> i32 {
        self.amount
    }
}
#[juniper::object (Context = Context)]
impl WorkerCondition {
    pub fn task_type(&self) -> TaskType {
        self.tt
    }
    pub fn amount(&self) -> i32 {
        self.amount
    }
}

impl Resources {
    fn resource(&self, res: ResourceType) -> i32 {
        self.res
            .iter()
            .find(|(rt, _n)| *rt == res)
            .map(|(_rt, n)| *n as i32)
            .unwrap_or(0)
    }
}

impl From<Vec<(ResourceType, i64)>> for Resources {
    fn from(res: Vec<(ResourceType, i64)>) -> Self {
        Resources { res }
    }
}

impl QuestConditions {
    pub fn new(
        res: Resources,
        karma: Option<i64>,
        pop: Option<i64>,
        buildings: Vec<BuildingCondition>,
        worker: Vec<WorkerCondition>,
    ) -> Self {
        Self {
            res,
            karma: karma.map(|i| i as i32),
            pop: pop.map(|i| i as i32),
            buildings,
            worker,
        }
    }
}

impl From<QuestWorkerCondition> for WorkerCondition {
    fn from(qwc: QuestWorkerCondition) -> Self {
        Self {
            tt: qwc.task_type,
            amount: qwc.amount as i32,
        }
    }
}
impl From<QuestBuildingCondition> for BuildingCondition {
    fn from(qbc: QuestBuildingCondition) -> Self {
        Self {
            bt: qbc.building_type,
            amount: qbc.amount as i32,
        }
    }
}
