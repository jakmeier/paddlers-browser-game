pub(super) mod attack_funnel;
pub(super) mod attack_spawn;
pub(super) mod economy_worker;
pub(super) mod event;
mod event_queue;
mod town_defence;
pub(super) mod town_worker;

use crate::db::*;
use crate::game_master::attack_spawn::{AttackSpawner, SendAnarchistAttack};
use actix::prelude::*;
use chrono::NaiveDateTime;
use paddlers_shared_lib::game_mechanics::hobos::HoboLevel;
use paddlers_shared_lib::game_mechanics::town::TOWN_X;
use paddlers_shared_lib::prelude::Player;
use paddlers_shared_lib::prelude::VillageKey;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::sql_db::keys::SqlKey;
use paddlers_shared_lib::story::story_state::StoryState;
use rand::RngCore;
use std::time::Duration;

pub struct GameMaster {
    last_attack: NaiveDateTime,
    dbpool: Pool,
    attacker_addr: Addr<AttackSpawner>,
}
impl GameMaster {
    pub fn new(dbpool: Pool, attacker_addr: &Addr<AttackSpawner>) -> Self {
        GameMaster {
            last_attack: NaiveDateTime::from_timestamp(0, 0),
            dbpool: dbpool,
            attacker_addr: attacker_addr.clone(),
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

        let now = chrono::Utc::now().naive_utc();
        if now - self.last_attack >= chrono::Duration::seconds(40) {
            self.last_attack = now;
            let mut rng = rand::thread_rng();
            let random_number = rng.next_u64() as usize;
            for village in db.all_player_villages() {
                let vid = village.key();
                let ongoing_attacks = db.attacks_count(vid, None);

                if should_send_attack(ongoing_attacks, random_number, vid) {
                    if let Some(player_info) = db.player_by_village(vid) {
                        if let Some(anarchist_level) = repetitive_attack_strength(&player_info) {
                            self.attacker_addr
                                .try_send(SendAnarchistAttack {
                                    village: vid,
                                    level: anarchist_level,
                                })
                                .expect("send failed");
                        }
                    }
                }
            }
        }

        ctx.run_later(Duration::from_secs(1), Self::game_cycle);
    }
}

// TODO [0.1.5]: Define this in specification document and/or integrate with wiki
fn should_send_attack(ongoing_attacks: usize, random_number: usize, vid: VillageKey) -> bool {
    match ongoing_attacks {
        0 => true,
        n => (random_number + vid.0 as usize) % (n * n + 9) == 0,
    }
}
// TODO [0.1.5]: Define this in specification document and/or integrate with wiki
fn repetitive_attack_strength(player: &Player) -> Option<HoboLevel> {
    match player.story_state {
        StoryState::Initialized
        | StoryState::ServantAccepted
        | StoryState::TempleBuilt
        | StoryState::VisitorArrived => None,
        StoryState::FirstVisitorWelcomed | StoryState::FlowerPlanted => Some(HoboLevel::zero()),
        StoryState::MoreHappyVisitors
        | StoryState::TreePlanted
        | StoryState::StickGatheringStationBuild
        | StoryState::GatheringSticks => Some(HoboLevel::anarchist(player.karma)),
    }
}

// TODO: Efficiently check only required attacks
fn check_attacks(db: &DB) {
    for village in db.all_player_villages() {
        let attacks = db.attacks(village.key(), None);
        let now = chrono::Utc::now().naive_utc();
        for atk in attacks.iter() {
            if atk.arrival + chrono::Duration::seconds(2 * TOWN_X as i64) < now {
                db.maybe_evaluate_attack(atk, now);
            }
        }
    }
}
