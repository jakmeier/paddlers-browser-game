mod collect_quest;
mod messages;
pub use messages::*;

use crate::db::*;
use actix::prelude::*;
use paddlers_shared_lib::game_mechanics::worker::hero_max_mana;

/// This actor executes DB requests which can be done concurrent to
/// the request processing or game-master logic.
pub struct DbActor {
    dbpool: Pool,
}

impl Handler<DeferredDbStatement> for DbActor {
    type Result = ();
    fn handle(&mut self, msg: DeferredDbStatement, ctx: &mut SyncContext<Self>) {
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
                // Potential for optimization: Keep quest assignment cached in memory (probably in a separate actor)
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
            DeferredDbStatement::AddMana(player, added_mana) => {
                let PlayerHome(village) = self.handle(PlayerHomeLookup { player }, ctx);
                let db = self.db();
                if let Some(hero) = db.hero(village) {
                    db.add_worker_mana(hero.key(), added_mana as i32, hero_max_mana())
                } else {
                    eprintln!("Player has no hero: {:?}", player);
                }
            }
            DeferredDbStatement::UnlockCivPerk(player, perk) => {
                if let Err(e) = self.db().unlock_civ_perk(player, perk) {
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
            match db.add_karma(player.key(), report.karma) {
                Err(e) => eprintln!("Karma reward collection failed: {}", e),
                Ok(_) => {}
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
