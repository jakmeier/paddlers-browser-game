//! Module for all graph QL nodes (entities) inner queries
//! This module does NOT perform authorization, field checks must
//! be done in the public interface.

use super::*;
use juniper;
use juniper::FieldResult;

#[juniper::object (Context = Context)]
impl GqlAttack {
    fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn units(&self, ctx: &Context) -> FieldResult<Vec<GqlHobo>> {
        Ok(ctx
            .db
            .attack_hobos(&self.0)
            .into_iter()
            .map(GqlHobo)
            .collect())
    }
    fn departure(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.departure)
    }
    fn arrival(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.arrival)
    }
}


#[juniper::object (Context = Context)]
impl GqlBuilding {
    fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn x(&self) -> i32 {
        self.0.x
    }
    fn y(&self) -> i32 {
        self.0.y
    }
    fn building_type(&self) -> &paddlers_shared_lib::models::BuildingType {
        &self.0.building_type
    }
    fn building_range(&self) -> Option<f64> {
        self.0.building_range.map(f64::from)
    }
    fn attack_power(&self) -> Option<f64> {
        self.0.attack_power.map(f64::from)
    }
    fn attacks_per_cycle(&self) -> Option<i32> {
        self.0.attacks_per_cycle
    }
    fn creation(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.creation)
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
        ctx.db
            .worker_tasks(WorkerKey(self.0.id))
            .into_iter()
            .map(GqlTask::authorized ) // Inherited authorization
            .collect()
    }

    fn abilities(&self, ctx: &Context) -> Vec<GqlAbility> {
        ctx.db
            .worker_abilities(WorkerKey(self.0.id))
            .into_iter()
            .map( GqlAbility::authorized ) // Inherited authorization
            .collect()
    }

    fn level(&self) -> i32 {
        self.0.level
    }

    fn experience(&self) -> i32 {
        self.0.exp
    }
}
