use diesel::{PgConnection, QueryResult, RunQueryDsl};
use paddlers_shared_lib::{
    prelude::{
        BuildingType, NewQuest, NewQuestBuildingCondition, NewQuestResCondition, NewQuestResReward,
        NewQuestWorkerCondition, Quest, ResourceType, TaskType,
    },
    schema::{
        quest_building_conditions, quest_res_conditions, quest_res_rewards,
        quest_worker_conditions, quests,
    },
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct QuestDefinition {
    pub quest_key: String,
    pub follow_up_quest: Option<String>,
    pub condition: QuestConditions,
    pub reward: QuestRewards,
    pub karma_condition: Option<i64>,
    pub pop_condition: Option<i64>,
}

#[derive(Deserialize)]
pub struct QuestConditions {
    pub buildings: Option<HashMap<BuildingType, i64>>,
    pub resources: Option<HashMap<ResourceType, i64>>,
    pub workers: Option<HashMap<TaskType, i64>>,
}

#[derive(Deserialize)]
pub struct QuestRewards {
    pub resources: Option<HashMap<ResourceType, i64>>,
}

impl QuestDefinition {
    pub fn upload(self, db: &PgConnection) -> QueryResult<()> {
        let quest = NewQuest {
            quest_key: self.quest_key,
            follow_up_quest: self.follow_up_quest,
            karma_condition: self.karma_condition,
            pop_condition: self.pop_condition,
        };
        let quest = diesel::insert_into(quests::dsl::quests)
            .values(&quest)
            .get_result::<Quest>(db)?;
        let quest_id = quest.id;

        // Conditions
        for (building_type, amount) in self.condition.buildings.into_iter().flatten() {
            let new = NewQuestBuildingCondition {
                quest_id,
                building_type,
                amount,
            };
            diesel::insert_into(quest_building_conditions::dsl::quest_building_conditions)
                .values(new)
                .execute(db)?;
        }
        for (resource_type, amount) in self.condition.resources.into_iter().flatten() {
            let new = NewQuestResCondition {
                quest_id,
                resource_type,
                amount,
            };
            diesel::insert_into(quest_res_conditions::dsl::quest_res_conditions)
                .values(new)
                .execute(db)?;
        }
        for (task_type, amount) in self.condition.workers.into_iter().flatten() {
            let new = NewQuestWorkerCondition {
                quest_id,
                task_type,
                amount,
            };
            diesel::insert_into(quest_worker_conditions::dsl::quest_worker_conditions)
                .values(new)
                .execute(db)?;
        }

        // Rewards
        for (resource_type, amount) in self.reward.resources.into_iter().flatten() {
            let new = NewQuestResReward {
                quest_id,
                resource_type,
                amount,
            };
            diesel::insert_into(quest_res_rewards::dsl::quest_res_rewards)
                .values(new)
                .execute(db)?;
        }

        Ok(())
    }
}
