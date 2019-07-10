use chrono::NaiveDateTime;
use std::sync::Arc;
use super::DbConn;
use juniper;
use juniper::FieldResult;
use db_lib::sql::GameDB;

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
    /// WIP for testing
    fn attacks(ctx: &Context) -> FieldResult<Vec<GqlAttack>> {
        Ok(
            ctx.db.attacks().into_iter().map(GqlAttack::from).collect()
        )
    }
    /// WIP for testing (should have at least a town filter)
    fn buildings(ctx: &Context) -> FieldResult<Vec<GqlBuilding>> {
        Ok(
            ctx.db.buildings().into_iter().map(GqlBuilding::from).collect()
        )
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

pub struct GqlUnit(db_lib::models::Unit);

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

pub struct GqlAttack(db_lib::models::Attack);
impl From<db_lib::models::Attack> for GqlAttack {
    fn from(inner: db_lib::models::Attack) -> Self {
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
    fn departure(&self) -> &NaiveDateTime {
        &self.0.departure
    }
    fn arrival(&self) -> &NaiveDateTime {
        &self.0.arrival
    }
}

pub struct GqlBuilding(db_lib::models::Building);
impl From<db_lib::models::Building> for GqlBuilding {
    fn from(inner: db_lib::models::Building) -> Self {
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
    fn building_range(&self) -> Option<f64> {
        self.0.building_range.map(f64::from)
    }
    fn attack_power(&self) -> Option<f64> {
        self.0.attack_power.map(f64::from)
    }
    fn attacks_per_cycle(&self) -> Option<i32> {
        self.0.attacks_per_cycle
    }
}
