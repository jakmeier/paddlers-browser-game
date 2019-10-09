mod town_defence;
mod event_queue;
pub (super) mod attack_spawn;
pub (super) mod event;
pub (super) mod town_worker;
pub (super) mod economy_worker;

use actix::prelude::*;
use std::time::Duration;
use chrono::{NaiveDateTime};
use crate::db::*;
use paddlers_shared_lib::sql::GameDB;
use paddlers_shared_lib::sql_db::keys::SqlKey;
use crate::game_master::attack_spawn::{AttackSpawner, AttackTarget};

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
            for village in db.all_player_villages() {
                self.attacker_addr.do_send(AttackTarget(village.key()));
            }
        }

        ctx.run_later(Duration::from_secs(1), Self::game_cycle);
    }
}


// TODO: Efficiently check only required attacks
fn check_attacks(db: &DB) {
    for village in db.all_player_villages() {
        let attacks = db.attacks(village.key(), None);
        let now = chrono::Utc::now().naive_utc();
        for atk in attacks.iter() {
            if atk.arrival < now {
                db.maybe_attack_now(atk, now -  atk.arrival);
            }
        }
    }
}