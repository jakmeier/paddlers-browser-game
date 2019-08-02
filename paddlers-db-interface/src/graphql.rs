use chrono::{NaiveDateTime};
use std::sync::Arc;
use super::DbConn;
use juniper;
use juniper::FieldResult;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::graphql_types::*;

pub struct Mutation;
pub struct Query;

pub struct Context { db: Arc<DbConn> }
// Necessary to make a DB connection available in GraphQL resolvers
impl juniper::Context for Context {}
impl From<DbConn> for Context {
    fn from(db: DbConn) -> Self {
        Context { db: Arc::new(db) }
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation)
}



#[juniper::object(
    Context = Context,
)]
impl Query {
    fn village(ctx: &Context, village_id: i32) -> FieldResult<GqlVillage> {
        Ok(GqlVillage{id: village_id as i64})
    }
    fn unit(ctx: &Context, unit_id: i32) -> FieldResult<GqlUnit> {
        Ok(GqlUnit(ctx.db.unit(unit_id as i64).ok_or("No such unit exists")?))
    }
}

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    /// You might think this is silly and has no reason to be here.
    /// You would be right for the former but wring for the latter.
    /// Why? GraphiQL is not able to display a schema with an empty Mutation.
    fn say_hi() -> &str {
        "Hi!"
    }
}

pub struct GqlUnit(paddlers_shared_lib::models::Unit);

#[juniper::object (Context = Context)]
impl GqlUnit {
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
    // TODO: Proper type handling
    pub fn hp(&self) -> i32 {
        self.0.hp as i32
    }
    // TODO: Proper type handling
    pub fn speed(&self) -> f64 {
        self.0.speed as f64
    }
    pub fn tasks(&self, ctx: &Context) -> Vec<GqlTask> {
        ctx.db.unit_tasks(self.0.id).into_iter().map(|t| GqlTask(t)).collect()
    }
}

pub struct GqlAttack(paddlers_shared_lib::models::Attack);
impl From<paddlers_shared_lib::models::Attack> for GqlAttack {
    fn from(inner: paddlers_shared_lib::models::Attack) -> Self {
        GqlAttack(inner)
    }
}

#[juniper::object (Context = Context)]
impl GqlAttack {
    fn id(&self) -> juniper::ID {
        self.0.id.to_string().into()
    }
    fn units(&self, ctx: &Context) -> FieldResult<Vec<GqlUnit>> {
        Ok(
            ctx.db.attack_units(&self.0).into_iter().map(|u| GqlUnit(u)).collect()
        )
    }
    fn departure(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.departure)
    }
    fn arrival(&self) -> FieldResult<GqlTimestamp> {
        datetime(&self.0.arrival)
    }
}

pub struct GqlBuilding(paddlers_shared_lib::models::Building);
impl From<paddlers_shared_lib::models::Building> for GqlBuilding {
    fn from(inner: paddlers_shared_lib::models::Building) -> Self {
        GqlBuilding(inner)
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

fn datetime(dt: &NaiveDateTime) -> FieldResult<GqlTimestamp> {
    Ok(GqlTimestamp::from_chrono(dt))
}

// TODO: Back this with DB (Only necessary once there is more than one village)
pub struct GqlVillage{
    id: i64,
}
#[juniper::object (Context = Context)]
impl GqlVillage {
    fn sticks(&self, ctx: &Context) -> i32 {
        ctx.db.resource(ResourceType::Sticks) as i32
    }
    fn feathers(&self, ctx: &Context) -> i32 {
        ctx.db.resource(ResourceType::Feathers) as i32
    }
    fn logs(&self, ctx: &Context) -> i32 {
        ctx.db.resource(ResourceType::Logs) as i32
    }
    fn units(&self, ctx: &Context) -> Vec<GqlUnit> {
        ctx.db.units(self.id).into_iter().map(|u| GqlUnit(u)).collect()
    }
    fn buildings(&self, ctx: &Context) -> FieldResult<Vec<GqlBuilding>> {
        // TODO: Filter for village
        Ok(
            ctx.db.buildings().into_iter().map(GqlBuilding::from).collect()
        )
    }
    #[graphql(
        arguments(
            min_id(
            description = "Response only contains attacks with id >= min_id",
            )
        )
    )]
    fn attacks(&self, ctx: &Context, min_id: Option<i32>) -> FieldResult<Vec<GqlAttack>> {
        // TODO: Filter for village
        Ok(
            ctx.db.attacks(min_id.map(i64::from)).into_iter().map(GqlAttack::from).collect()
        )
    }
}


pub struct GqlTask(paddlers_shared_lib::models::Task);
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
}
