mod collect_quest;
mod messages;
use diesel::QueryResult;
pub use messages::*;

use crate::db::*;
use actix::prelude::*;
use paddlers_shared_lib::story::story_state::StoryState;

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
            DeferredDbStatement::AssignQuest(player, quest_name) => {
                let db = self.db();
                // Potential for optimazion: Keep quest assignment cached in memory (probably in a separate actor)
                if let Err(e) = db
                    .quest_by_name(quest_name)
                    .and_then(|quest| db.assign_player_quest(player, quest.key()))
                {
                    eprintln!("Player update failed: {}", e);
                }
            }
            DeferredDbStatement::PlayerUpdate(player, new_story_state) => {
                if let Err(e) = self.db().set_story_state(player, new_story_state) {
                    eprintln!("Player update failed: {}", e);
                }
            }
        }
    }
}

impl Handler<PlayerHomeLookup> for DbActor {
    type Result = PlayerHome;
    fn handle(&mut self, msg: PlayerHomeLookup, _ctx: &mut SyncContext<Self>) -> Self::Result {
        PlayerHome(
            self.db()
                .player_villages(msg.player)
                .pop()
                .expect("player must have at least one village")
                .key(),
        )
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
            match db
                .add_karma(player.key(), report.karma)
                .and_then(|player| self.update_player_karma_progress(&player, report.karma))
            {
                Err(e) => eprintln!("Karma reward collection failed: {}", e),
                Ok(()) => {}
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
    fn update_player_karma_progress(&self, player: &Player, new_karma: i64) -> QueryResult<()> {
        // Note: This is not the ideal place nor the ideal way of handling this.
        // Consider improving this in [0.1.5]
        // First karma gained
        if player.karma - new_karma == 0 {
            self.db()
                .set_story_state(player.key(), StoryState::FirstVisitorWelcomed)?;
        }

        Ok(())
    }
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
    fn started(&mut self, _ctx: &mut SyncContext<Self>) {}

    fn stopped(&mut self, _ctx: &mut SyncContext<Self>) {
        eprintln!("Stopped DB actor");
    }
}
