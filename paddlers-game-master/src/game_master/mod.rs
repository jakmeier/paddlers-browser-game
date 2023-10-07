pub(super) mod attack_funnel;
pub(super) mod attack_spawn;
pub(super) mod economy_worker;
pub(super) mod event;
mod event_queue;
pub(super) mod story_worker;
mod taxes;
mod town_defence;
pub(super) mod town_worker;

use crate::db::*;
use crate::game_master::attack_spawn::{AttackSpawner, SendAnarchistAttack};
use actix::prelude::*;
use chrono::NaiveDateTime;
use paddlers_shared_lib::game_mechanics::town::TOWN_X;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::specification_types::HoboLevel;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::sql_db::keys::SqlKey;
use paddlers_shared_lib::story::story_state::StoryState;
use rand::RngCore;
use std::time::Duration;

pub struct GameMaster {
    last_attack: NaiveDateTime,
    dbpool: Pool,
    attacker_addr: Addr<AttackSpawner>,
    current_batch: Option<VillageBatch>,
}
/// Keeps partial progress when checking if an attack to villages is required
struct VillageBatch {
    villages: Vec<Village>,
    seed: usize,
}
impl GameMaster {
    pub fn new(dbpool: Pool, attacker_addr: &Addr<AttackSpawner>) -> Self {
        GameMaster {
            last_attack: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            dbpool: dbpool,
            attacker_addr: attacker_addr.clone(),
            current_batch: None,
        }
    }
}

impl Actor for GameMaster {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Game Master is alive");
        self.game_cycle(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Game Master is stopped");
    }
}

impl GameMaster {
    fn game_cycle(&mut self, ctx: &mut Context<Self>) {
        // println!("Cycle");

        let db: DB = (&self.dbpool).into();
        check_attacks(&db);

        if self.current_batch.is_none() {
            let now = chrono::Utc::now().naive_utc();
            if now - self.last_attack >= chrono::Duration::seconds(40) {
                self.last_attack = now;
                self.load_new_batch(&db);
            }
        }

        self.continue_batch(&db);

        ctx.run_later(Duration::from_secs(1), Self::game_cycle);
    }
    fn load_new_batch(&mut self, db: &DB) {
        let mut rng = rand::thread_rng();
        self.current_batch = Some(VillageBatch {
            villages: db.all_player_villages(),
            seed: rng.next_u64() as usize,
        });
    }
    // Continues working on batch until the attack mailbox is full
    fn continue_batch(&mut self, db: &DB) {
        if let Some(batch) = self.current_batch.as_mut() {
            while let Some(village) = batch.villages.pop() {
                let vid = village.key();
                let ongoing_attacks = db.attacks_count(vid, None);
                if should_send_attack(ongoing_attacks, batch.seed, vid) {
                    if let Some(player_info) = db.player_by_village(vid) {
                        if let Some(anarchist_level) = repetitive_attack_strength(&player_info) {
                            match self.attacker_addr.try_send(SendAnarchistAttack {
                                village: vid,
                                level: anarchist_level,
                            }) {
                                Err(SendError::Closed(_msg)) => panic!("Attack funnel closed"),
                                Err(SendError::Full(_msg)) => {
                                    batch.villages.push(village);
                                    return;
                                }
                                Ok(()) => { /* NOP */ }
                            }
                        }
                    }
                }
            }
            self.current_batch = None;
        }
    }
}

fn should_send_attack(ongoing_attacks: usize, random_number: usize, vid: VillageKey) -> bool {
    match ongoing_attacks {
        0 => true,
        n => (random_number + vid.0 as usize) % (n * n + 9) == 0,
    }
}
fn repetitive_attack_strength(player: &Player) -> Option<HoboLevel> {
    match player.story_state {
        StoryState::Initialized
        | StoryState::ServantAccepted
        | StoryState::TempleBuilt
        | StoryState::WatergateBuilt
        | StoryState::BuildingWatergate
        | StoryState::PickingPrimaryCivBonus
        | StoryState::SolvingPrimaryCivQuestPartA
        | StoryState::SolvingPrimaryCivQuestPartB
        | StoryState::SolvingSecondaryQuestA
        | StoryState::SolvingSecondaryQuestB
        | StoryState::DialogueBalanceA
        | StoryState::DialogueBalanceB
        | StoryState::VisitorQueued
        | StoryState::WelcomeVisitorQuestStarted
        | StoryState::VisitorArrived
        | StoryState::UnlockingInvitationPathA
        | StoryState::UnlockingInvitationPathB
        | StoryState::FirstVisitorWelcomed => None,
        StoryState::AllDone => Some(HoboLevel::anarchist(player.karma)),
    }
}

fn check_attacks(db: &DB) {
    for village in db.all_player_villages() {
        let attacks = db.attacks_that_entered(village.key(), None);
        let now = chrono::Utc::now().naive_utc();
        for atk in attacks.iter() {
            if let Some(fight_start) = atk.entered_destination {
                if fight_start + chrono::Duration::seconds(2 * TOWN_X as i64) < now {
                    db.maybe_evaluate_attack(atk, now);
                }
            }
        }
    }
}
