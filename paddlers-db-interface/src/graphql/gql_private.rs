//! Module for all graph QL nodes (entities) inner queries
//! This module does NOT perform authorization, field checks must
//! be done in the public interface.

use super::*;
use juniper;
use juniper::FieldResult;
use paddlers_shared_lib::story::story_state::StoryState;

#[juniper::object (Context = Context)]
impl GqlAttack {
    fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn units(&self, ctx: &Context) -> FieldResult<Vec<GqlAttackUnit>> {
        Ok(ctx
            .db()
            .attack_hobos_with_attack_info(&self.0)
            .into_iter()
            .map(|(hobo, info)| GqlAttackUnit(GqlHobo(hobo), GqlHoboAttackInfo(info)))
            .collect())
    }
    fn departure(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.departure)
    }
    fn arrival(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.arrival)
    }
    fn entered_village(&self) -> FieldResult<Option<GqlTimestamp>> {
        self.0
            .entered_destination
            .as_ref()
            .map(datetime)
            .transpose()
    }
    fn attacker(&self, ctx: &Context) -> FieldResult<Option<GqlPlayer>> {
        let db = ctx.db();
        Ok(self
            .0
            .origin_village_id
            .and_then(|vid| db.village(VillageKey(vid)))
            .and_then(|village| village.owner())
            .and_then(|pid| db.player(pid))
            .map(|player| GqlPlayer(player)))
    }
}
#[juniper::object (Context = Context)]
impl GqlAttackUnit {
    fn hobo(&self) -> &GqlHobo {
        &self.0
    }
    fn info(&self) -> &GqlHoboAttackInfo {
        &self.1
    }
}
#[juniper::object (Context = Context)]
impl GqlHoboAttackInfo {
    fn released(&self) -> FieldResult<Option<GqlTimestamp>> {
        Ok(self.0.released.as_ref().map(GqlTimestamp::from_chrono))
    }
    fn satisfied(&self) -> FieldResult<Option<bool>> {
        Ok(self.0.satisfied)
    }
}

#[juniper::object (Context = Context)]
impl GqlAttackReport {
    fn id(&self) -> juniper::ID {
        self.inner.id.to_string().into()
    }
    fn reported(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.inner.reported)
    }
    fn karma(&self) -> i32 {
        self.inner.karma as i32
    }
    fn resources(&self) -> &Resources {
        self.rewards.as_ref().unwrap()
    }
}
impl GqlAttackReport {
    pub fn load_rewards(&mut self, ctx: &Context) {
        if self.rewards.is_none() {
            let db = ctx.db();
            self.rewards = Some(db.rewards(self.inner.key()).into())
        }
    }
}

#[juniper::object (Context = Context)]
impl GqlTask {
    fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn x(&self) -> i32 {
        self.0.x
    }
    fn y(&self) -> i32 {
        self.0.y
    }
    fn task_type(&self) -> &paddlers_shared_lib::models::TaskType {
        &self.0.task_type
    }
    fn start_time(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.start_time)
    }
    fn hobo_target(&self) -> Option<i32> {
        self.0.target_hobo_id.map(|id| id as i32)
    }
}

#[juniper::object (Context = Context)]
impl GqlAbility {
    fn ability_type(&self) -> &paddlers_shared_lib::models::AbilityType {
        &self.0.ability_type
    }
    fn last_used(&self) -> Option<GqlTimestamp> {
        self.0.last_used.as_ref().map(GqlTimestamp::from_chrono)
    }
}

#[juniper::object (Context = Context)]
impl GqlEffect {
    pub fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn attribute(&self) -> &paddlers_shared_lib::models::HoboAttributeType {
        &self.0.attribute
    }
    fn start_time(&self) -> GqlTimestamp {
        GqlTimestamp::from_chrono(&self.0.start_time)
    }
    fn strength(&self) -> Option<i32> {
        self.0.strength
    }
}

#[juniper::object (Context = Context)]
impl GqlWorker {
    pub fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }

    pub fn unit_type(&self) -> &paddlers_shared_lib::models::UnitType {
        &self.0.unit_type
    }

    pub fn color(&self) -> &Option<paddlers_shared_lib::models::UnitColor> {
        &self.0.color
    }

    fn x(&self) -> i32 {
        self.0.x
    }

    fn y(&self) -> i32 {
        self.0.y
    }

    fn mana(&self) -> Option<i32> {
        self.0.mana
    }

    // TODO: Proper type handling
    pub fn speed(&self) -> f64 {
        self.0.speed as f64
    }

    pub fn tasks(&self, ctx: &Context) -> Vec<GqlTask> {
        ctx.db()
            .worker_tasks(WorkerKey(self.0.id))
            .into_iter()
            .map(GqlTask::authorized) // Inherited authorization
            .collect()
    }

    fn abilities(&self, ctx: &Context) -> Vec<GqlAbility> {
        ctx.db()
            .worker_abilities(WorkerKey(self.0.id))
            .into_iter()
            .map(GqlAbility::authorized) // Inherited authorization
            .collect()
    }

    fn level(&self) -> i32 {
        self.0.level
    }

    fn experience(&self) -> i32 {
        self.0.exp
    }
}

#[juniper::object (Context = Context)]
impl GqlQuest {
    pub fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    pub fn key(&self) -> &str {
        &self.0.quest_key
    }
    pub fn next_story_state(&self) -> &Option<StoryState> {
        &self.0.next_story_state
    }
    pub fn rewards(&self, ctx: &Context) -> Resources {
        ctx.db()
            .quest_res_rewards(QuestKey(self.0.id))
            .into_iter()
            .map(|r| (r.resource_type, r.amount))
            .collect::<Vec<_>>()
            .into()
    }
    pub fn conditions(&self, ctx: &Context) -> QuestConditions {
        let db = ctx.db();
        let key = QuestKey(self.0.id);
        let res = db
            .quest_res_conditions(key)
            .into_iter()
            .map(|r| (r.resource_type, r.amount))
            .collect::<Vec<_>>()
            .into();
        let buildings = db
            .quest_building_conditions(key)
            .into_iter()
            .map(Into::into)
            .collect();
        let worker = db
            .quest_worker_conditions(key)
            .into_iter()
            .map(Into::into)
            .collect();
        let karma = self.0.karma_condition;
        QuestConditions::new(res, karma, buildings, worker)
    }
}
