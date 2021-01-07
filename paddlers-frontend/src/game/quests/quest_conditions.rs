use crate::net::graphql::PlayerQuest;
use paddlers_shared_lib::prelude::*;

#[derive(Clone, Debug)]
pub struct ResourceCondition {
    t: ResourceType,
    amount: i64,
}

#[derive(Clone, Debug)]
pub struct BuildingCondition {
    t: BuildingType,
    amount: i64,
}
#[derive(Clone, Debug)]
pub struct WorkerCondition {
    t: TaskType,
    amount: i64,
}

impl ResourceCondition {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        let rcs = &quest.conditions.resources;
        if rcs.feathers > 0 {
            out.push(Self {
                t: ResourceType::Feathers,
                amount: rcs.feathers,
            });
        }
        if rcs.sticks > 0 {
            out.push(Self {
                t: ResourceType::Sticks,
                amount: rcs.sticks,
            });
        }
        if rcs.logs > 0 {
            out.push(Self {
                t: ResourceType::Logs,
                amount: rcs.logs,
            });
        }
        out
    }
}

impl BuildingCondition {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.buildings {
            out.push(BuildingCondition {
                t: (&c.building_type).into(),
                amount: c.amount,
            })
        }
        out
    }
}

impl WorkerCondition {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.workers {
            out.push(WorkerCondition {
                t: (&c.task_type).into(),
                amount: c.amount,
            })
        }
        out
    }
}
