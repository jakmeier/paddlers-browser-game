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

pub struct AuthenticatedContext {
    db: Arc<DbConn>,
    user: Player,
    villages: Vec<VillageKey>,
}
pub struct UnauthenticatedContext {
    db: Arc<DbConn>,
}
pub enum Context {
    Public(UnauthenticatedContext),
    Authenticated(AuthenticatedContext),
}
// Necessary to make a DB connection available in GraphQL resolvers
impl juniper::Context for Context {}

impl Context {
    pub fn new(db: DbConn, user: Option<PadlUser>) -> Option<Self> {
        let conn = Arc::new(db);
        if let Some(user) = user {
            let player = conn.player_by_uuid(user.uuid)?;
            let vids = conn.player_villages(player.key()).into_iter().map(|v|v.key()).collect();
            Some(Context::Authenticated( AuthenticatedContext { db: conn, user: player, villages: vids }))
        } else {
            Some(Context::Public(UnauthenticatedContext{ db: conn }))
        }
    }
    pub fn db(&self) -> &Arc<DbConn> {
        match self {
            Context::Authenticated(ctx) => {
                &ctx.db
            },
            Context::Public(ctx) => {
                &ctx.db
            }
        }
    }
    pub fn authenticated(&self) -> Result<&AuthenticatedContext, ReadableInterfaceError> {
        match self {
            Context::Authenticated(ctx) => {
                Ok(ctx)
            },
            Context::Public(_) => {
                Err(ReadableInterfaceError::RequiresAuthentication)
            }
        }

    }
    fn check_user_key(&self, key: PlayerKey) -> Result<(), ReadableInterfaceError> {
        if key.0 == self.authenticated()?.user.id {
            Ok(())
        } else {
            Err(ReadableInterfaceError::NotAllowed)
        }
    }
    fn check_village_key(&self, key: VillageKey) -> Result<(), ReadableInterfaceError> {
        if self.authenticated()?.villages.contains(&key) {
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
    fn player(ctx: &Context, player_id: Option<i32>) -> FieldResult<GqlPlayer> {
        if let Some(i) = player_id {
            let key = PlayerKey(i as i64);
            let player = ctx.db().player(key).ok_or("No such player")?;
            Ok(GqlPlayer(player))
        } else {
            let player = ctx.authenticated()?.user.clone();
            Ok(GqlPlayer(player))
        }
    }
    // Object Visibility: public
    fn village(ctx: &Context, village_id: i32) -> FieldResult<GqlVillage> {
        let village = ctx.db().village(VillageKey(village_id as i64)).ok_or("No such village")?;
        Ok(GqlVillage(village))
    }
    // Object Visibility: user
    fn worker(ctx: &Context, worker_id: i32) -> FieldResult<GqlWorker> {
        Ok(GqlWorker::authorized(
            ctx.db()
                .worker_auth_by_player(WorkerKey(worker_id as i64), ctx.authenticated()?.user.key())
                .ok_or("No such unit visible")?,
        ))
    }
    // Object Visibility: user
    fn hobo(ctx: &Context, hobo_id: i32) -> FieldResult<GqlHobo> {
        Ok(GqlHobo(
            ctx.db().hobo(hobo_id as i64).ok_or("No such unit exists")?,
        ))
    }
    // Object Visibility: public
    fn map(low_x: i32, high_x: i32) -> GqlMapSlice {
        GqlMapSlice { low_x, high_x }
    }
    // Object Visibility: public
    // Returns up to 100 players starting from the given rank upwards
    fn scoreboard(ctx: &Context, rank_offset: i32) -> Vec<GqlPlayer> {
        ctx.db().players_sorted_by_karma(rank_offset as i64, 100)
            .into_iter()
            .map(GqlPlayer)
            .collect()
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