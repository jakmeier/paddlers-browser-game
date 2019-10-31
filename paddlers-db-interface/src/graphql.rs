//! Module for the GraphQL root query definition.


use super::DbConn;
use juniper;
use juniper::FieldResult;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::graphql_types::*;
use paddlers_shared_lib::user_authentication::PadlUser;
use std::sync::Arc;
use chrono::prelude::NaiveDateTime;

mod gql_err;
mod gql_public;
pub mod gql_private;

use gql_err::ReadableInterfaceError;
use gql_public::*;

pub struct Mutation;
pub struct Query;

pub struct Context {
    db: Arc<DbConn>,
    user: PlayerKey,
    villages: Vec<VillageKey>,
}
// Necessary to make a DB connection available in GraphQL resolvers
impl juniper::Context for Context {}

impl Context {
    pub fn new(db: DbConn, user: PadlUser) -> Option<Self> {
        let id = db.player_by_uuid(user.uuid)?.key();
        let vids = db.player_villages(id).into_iter().map(|v|v.key()).collect();
        let conn = Arc::new(db);
        Some(Context { db: conn, user: id, villages: vids })
    }
    fn check_user_key(&self, key: PlayerKey) -> Result<(), ReadableInterfaceError> {
        if key.0 == self.user.0 {
            Ok(())
        } else {
            Err(ReadableInterfaceError::NotAllowed)
        }
    }
    fn check_village_key(&self, key: VillageKey) -> Result<(), ReadableInterfaceError> {
        if self.villages.contains(&key) {
            Ok(())
        } else {
            Err(ReadableInterfaceError::NotAllowed)
        }
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub fn new_schema() -> Schema {
    Schema::new(Query, Mutation)
}

#[juniper::object(Context = Context)]
impl Query {
    // Object Visibility: public
    fn player(ctx: &Context, player_id: i32) -> FieldResult<GqlPlayer> {
        let player = ctx.db.player(player_id as i64).ok_or("No such player")?;
        Ok(GqlPlayer(player))
    }
    // Object Visibility: public
    fn village(ctx: &Context, village_id: i32) -> FieldResult<GqlVillage> {
        let village = ctx.db.village(village_id as i64).ok_or("No such village")?;
        Ok(GqlVillage(village))
    }
    // Object Visibility: user
    fn worker(ctx: &Context, worker_id: i32) -> FieldResult<GqlWorker> {
        Ok(GqlWorker::authorized(
            ctx.db
                .worker_auth_by_player(WorkerKey(worker_id as i64), ctx.user)
                .ok_or("No such unit visible")?,
        ))
    }
    // Object Visibility: user
    fn hobo(ctx: &Context, hobo_id: i32) -> FieldResult<GqlHobo> {
        Ok(GqlHobo(
            ctx.db.hobo(hobo_id as i64).ok_or("No such unit exists")?,
        ))
    }
    // Object Visibility: public
    fn map(low_x: i32, high_x: i32) -> GqlMapSlice {
        GqlMapSlice { low_x, high_x }
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

fn datetime(dt: &NaiveDateTime) -> FieldResult<GqlTimestamp> {
    Ok(GqlTimestamp::from_chrono(dt))
}