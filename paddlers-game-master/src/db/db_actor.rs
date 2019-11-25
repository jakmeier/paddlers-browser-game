use crate::db::*;
use actix::prelude::*;

/// This actor executes DB requests which can be done concurrent to
/// the request processing or game-master logic.
pub struct DbActor {
    dbpool: Pool,
}

#[derive(Debug)]
/// Deferred DB requests should not be dependent on the state of the DB
/// and instead be logically guaranteed to work. For example, the resource 
/// price should already be payed before-hand.
pub enum DeferredDbStatement {
    NewProphet(VillageKey),
}

impl Message for DeferredDbStatement {
    type Result = ();
}
impl Handler<DeferredDbStatement> for DbActor {
    type Result = ();
    fn handle(&mut self, msg: DeferredDbStatement, _ctx: &mut Context<Self>) {
        match msg {
            DeferredDbStatement::NewProphet(village) => {
                self.db().add_prophet(village);
            }
        }
    }
}

impl DbActor {
    pub fn new(dbpool: Pool) -> Self { 
        DbActor {
            dbpool: dbpool,
        }
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }
}

impl Actor for DbActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Context<Self>) {
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        eprintln!("Stopped DB actor");
    }
}