use chrono::{NaiveDateTime, Utc};
use chrono::offset::TimeZone;
use std::sync::Arc;
use super::DbConn;
use juniper;
use juniper::FieldResult;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::models::*;

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
    /// WIP for testing
    fn units(ctx: &Context) -> FieldResult<Vec<GqlUnit>> {
        Ok(
            ctx.db.units().into_iter().map(|u| GqlUnit(u)).collect()
        )
    }
    /// WIP for testing (lacks village filter)
    #[graphql(
        arguments(
            min_id(
            description = "Response only contains attacks with id >= min_id",
            )
        )
    )]
    fn attacks(ctx: &Context, min_id: Option<i32>) -> FieldResult<Vec<GqlAttack>> {
        Ok(
            ctx.db.attacks(min_id.map(i64::from)).into_iter().map(GqlAttack::from).collect()
        )
    }
    /// WIP for testing (should have at least a town filter)
    fn buildings(ctx: &Context) -> FieldResult<Vec<GqlBuilding>> {
        Ok(
            ctx.db.buildings().into_iter().map(GqlBuilding::from).collect()
        )
    }
    /// WIP for testing
    fn village(ctx: &Context) -> FieldResult<GqlVillage> {
        Ok(GqlVillage)
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
    pub fn sprite(&self) -> &str {
        &self.0.sprite
    }
    // TODO: Proper type handling
    pub fn hp(&self) -> i32 {
        self.0.hp as i32
    }
    // TODO: Proper type handling
    pub fn speed(&self) -> f64 {
        self.0.speed as f64
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
    fn departure(&self) -> FieldResult<NaiveDateTime> {
        datetime(&self.0.departure)
    }
    fn arrival(&self) -> FieldResult<NaiveDateTime> {
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
    fn creation(&self) -> FieldResult<NaiveDateTime> {
        datetime(&self.0.creation)
    }
}

fn datetime(dt: &NaiveDateTime) -> FieldResult<NaiveDateTime> {
    let date = Utc.from_local_datetime(dt);
    if date.single().is_none() {
        return Err("Datetime from DB is not unique".into());
    }
    Ok(
        date.unwrap().naive_utc()
    )
}


pub struct GqlVillage;
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

}
