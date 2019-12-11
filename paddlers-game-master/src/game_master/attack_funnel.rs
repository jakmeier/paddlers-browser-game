//! Funnels planned attacks
//! 
//! All attacks on villages should go through this funnel.
//! This module should then guarantee that no two attacks reach a town at the same time
//! and that no hobo is involved in more than one attack at the time.

use chrono::NaiveDateTime;
use actix::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::game_mechanics::map::map_distance;
use crate::db::*;
use std::ops::Add;

pub struct AttackFunnel {
    dbpool: Pool,
    db_actor: Addr<DbActor>,
}

impl Actor for AttackFunnel {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Attack Funnel is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Attack Funnel is stopped");
    }
}

pub struct PlannedAttack{
    pub origin_village: Option<Village>,
    pub destination_village: Village,
    pub hobos: Vec<Hobo>,
}
impl Message for PlannedAttack {
    type Result = ();
}


impl Handler<PlannedAttack> for AttackFunnel {
    type Result = ();

    fn handle(&mut self, msg: PlannedAttack, _ctx: &mut Context<Self>) -> Self::Result {
        let db = self.db();
        let vid = msg.destination_village.key();

        // TODO (Correctness): Somehow efficiently check that hobos are not attacking already
        let unit_count = msg.hobos.len();
        let hobos = msg.hobos.into_iter().map(|h|h.key()).collect();

        let min_secs = 15;
        let travel_time = if let Some(v0) = msg.origin_village {
            let v1 = msg.destination_village;
            let distance = map_distance((v0.x,v0.y), (v1.x,v1.y));
            let seconds = 20.0 * distance;
            chrono::Duration::seconds(min_secs + seconds as i64)
        } else {
            chrono::Duration::seconds(min_secs)
        };
        let now = chrono::Utc::now().naive_utc();
        let earliest_arrival = now.add(travel_time);
        let arrival = Self::next_timeslot(&db, vid, unit_count, earliest_arrival);
        let attack = NewAttack {
            departure: now,
            arrival: arrival,
            origin_village_id: msg.origin_village.map(|k|k.id),
            destination_village_id: msg.destination_village.id,
        };

        let pa = ScheduledAttack { attack, hobos };
        self.db_actor.try_send(DeferredDbStatement::NewAttack(pa)).expect("Sending attack failed");
    }
}

impl AttackFunnel {
    pub fn new(dbpool: Pool, db_actor: Addr<DbActor>) -> Self {
        AttackFunnel {
            dbpool: dbpool,
            db_actor: db_actor,
        }
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }
    fn next_timeslot(db: &DB, vid: VillageKey, unit_count: usize, mut earliest: NaiveDateTime) -> NaiveDateTime {
        // TODO (Optimization): These are potentially many DB queries

        // Query returns attacks sorted by arrival date
        let already_running_attacks = db.attacks(vid, None);
        let duration = Self::attack_duration(unit_count); 
        let mut i = 0;
        let len = already_running_attacks.len();
        while i < len {
            let atk = &already_running_attacks[i];
            let n = db.attack_hobos(atk).len();
            let d = Self::attack_duration(n);
            if atk.arrival + d <= earliest {
                // No conflict with i, i is earlier than new attack
                i+= 1;
            }
            else if atk.arrival < earliest + duration {
                // Conflict with i, need to delay new attack to be after i
                i+= 1;
                earliest = atk.arrival + d;
            } else {
                // No overlap and i is entirely afterwards
                //  => thanks to sorted input we can stop here
                break;
            }
        }
        earliest
    }
    fn attack_duration(units: usize) -> chrono::Duration {
        // Assumptions: 
        //  A) ~0.2 speed <=> 5s per tile
        //  B) Two units parallel to each other
        let seconds = (units+1) * 5 / 2;
        chrono::Duration::seconds(seconds as i64)
    }
}