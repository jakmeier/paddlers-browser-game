mod messages;
pub use messages::*;

use crate::db::*;
use actix::prelude::*;

/// This actor executes DB requests which can be done concurrent to
/// the request processing or game-master logic.
pub struct DbActor {
    dbpool: Pool,
}

impl Handler<DeferredDbStatement> for DbActor {
    type Result = ();
    fn handle(&mut self, msg: DeferredDbStatement, _ctx: &mut SyncContext<Self>) {
        match msg {
            DeferredDbStatement::NewProphet(village) => {
                self.db().add_prophet(village);
            }
            DeferredDbStatement::NewAttack(planned_atk) => {
                let attack = self.db().insert_attack(&planned_atk.attack);
                for hobo in planned_atk.hobos.iter() {
                    let atu = AttackToHobo {
                        attack_id: attack.id,
                        hobo_id: hobo.num(),
                        satisfied: None,
                        released: None,
                    };
                    self.db().insert_attack_to_hobo(&atu);
                }
            }
        }
    }
}

impl Handler<NewHoboMessage> for DbActor {
    type Result = NewHoboResponse;
    fn handle(&mut self, msg: NewHoboMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let hobo = self.db().insert_hobo(&msg.0);
        NewHoboResponse(hobo)
    }
}

impl Handler<CollectReportRewardsMessage> for DbActor {
    type Result = ();
    fn handle(
        &mut self,
        msg: CollectReportRewardsMessage,
        _ctx: &mut SyncContext<Self>,
    ) -> Self::Result {
        let report = msg.0;
        let village = report.village();
        let db = self.db();
        for (resource_type, n) in db.rewards(report.key()) {
            if let Err(e) = db.add_resource(resource_type, village, n) {
                eprintln!("Reward collection failed: {}", e);
            }
        }
        if let Some(player) = db.player_by_village(village) {
            if let Err(e) = db.add_karma(player.key(), report.karma) {
                eprintln!("Karma reward collection failed: {}", e);
            }
        }
        db.delete_visit_report(&report);
    }
}

impl DbActor {
    pub fn new(dbpool: Pool) -> Self {
        DbActor { dbpool: dbpool }
    }
    fn db(&self) -> DB {
        (&self.dbpool).into()
    }
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut SyncContext<Self>) {}

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        eprintln!("Stopped DB actor");
    }
}
