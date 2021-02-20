//! Funnels planned attacks
//!
//! All attacks on villages should go through this funnel.
//! This module should then guarantee some consistencies about attacks in a village.
//! For now, the consistency rules are:
//!     - No two attacks reach a town at the same time
//!     - No hobo is involved in more than one attack at the time
//!     - The maximum number of resting hobos is not surpassed

use crate::game_master::event::Event;
use crate::game_master::town_defence::AttackingHobo;
use crate::game_master::town_worker::{TownWorker, TownWorkerEventMsg};
use crate::{db::*, town_view::TownView};
use actix::prelude::*;
use chrono::{offset::TimeZone, NaiveDateTime, Utc};
use paddlers_shared_lib::game_mechanics::{map::map_distance, town::defence::IAttackingHobo};
use paddlers_shared_lib::prelude::*;
use std::ops::Add;

const MIN_DELAY_BETWEEN_ATTACKS: i64 = 20;

pub struct AttackFunnel {
    dbpool: Pool,
    db_actor: Addr<DbActor>,
    town_worker: Addr<TownWorker>,
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

pub struct PlannedAttack {
    pub origin_village: Option<Village>,
    pub destination_village: Village,
    pub hobos: Vec<Hobo>,
    pub fixed_travel_time_s: Option<i32>,
    pub subject_to_visitor_queue_limit: bool,
}
impl Message for PlannedAttack {
    type Result = ();
}

impl Handler<PlannedAttack> for AttackFunnel {
    type Result = ();

    fn handle(&mut self, msg: PlannedAttack, _ctx: &mut Context<Self>) -> Self::Result {
        let db = self.db();
        let vid = msg.destination_village.key();

        // Check that the visitor queue capacity is respected
        if msg.subject_to_visitor_queue_limit {
            let active_attacks = db.attacks(vid, None).len();
            let town_context = TownView::load_village(&db, vid);
            if town_context.state.visitor_capacity() <= active_attacks {
                eprintln!("Attempted to invite more than allowed. Request ignored.");
                return;
            }
        }

        // Checks and attack creation must be sequential
        // TODO (Correctness): Somehow efficiently check that hobos are not attacking already
        let unit_count = msg.hobos.len();
        let unhurried = msg
            .hobos
            .iter()
            .cloned()
            .filter(|h| !h.hurried)
            .collect::<Vec<_>>();
        let hobos = msg.hobos.into_iter().map(|h| h.key()).collect();

        let travel_time;
        if let Some(s) = msg.fixed_travel_time_s {
            travel_time = s as i64;
        } else if let Some(v0) = msg.origin_village {
            let v1 = msg.destination_village;
            let distance = map_distance((v0.x, v0.y), (v1.x, v1.y));
            let seconds = 20.0 * distance;
            travel_time = MIN_DELAY_BETWEEN_ATTACKS.max(seconds as i64);
        } else {
            travel_time = MIN_DELAY_BETWEEN_ATTACKS;
        }
        let now = chrono::Utc::now().naive_utc();
        let earliest_arrival = now.add(chrono::Duration::seconds(travel_time));
        let arrival = Self::next_timeslot(&db, vid, unit_count, earliest_arrival);
        let attack = NewAttack {
            departure: now,
            arrival: arrival,
            origin_village_id: msg.origin_village.map(|k| k.id),
            destination_village_id: msg.destination_village.id,
        };

        // Put new attack in DB
        let pa = ScheduledAttack { attack, hobos };
        self.db_actor.do_send(DeferredDbStatement::NewAttack(pa));
        // Everything until here must be sequential
        // FIXME: the deferred DB statement is not sequential and it would be possible that two attacks receive the same time slot!

        // Validate the resting queue the attack arrives, unless there is no unhurried hobo
        if unhurried.len() > 0 {
            let delayed_event = Event::CheckRestingVisitors {
                village_id: msg.destination_village.key(),
            };
            let time = arrival;
            self.town_worker.do_send(TownWorkerEventMsg(
                delayed_event,
                Utc.from_utc_datetime(&time),
            ));
        }
        // For all unhurried hobos, the hp should be checked when they reach the resting place
        for hobo in unhurried {
            let delayed_event = Event::CheckVisitorHp {
                hobo_id: hobo.key(),
            };
            let swim_time: chrono::Duration =
                AttackingHobo::s_time_until_resting(hobo.speed).into();
            let event_time = arrival + swim_time;
            self.town_worker.do_send(TownWorkerEventMsg(
                delayed_event,
                Utc.from_utc_datetime(&event_time),
            ));
        }
    }
}

impl AttackFunnel {
    pub fn new(dbpool: Pool, db_actor: Addr<DbActor>, town_worker: Addr<TownWorker>) -> Self {
        AttackFunnel {
            dbpool,
            db_actor,
            town_worker,
        }
    }
    fn db(&self) -> DB {
        (&self.dbpool).into()
    }
    fn next_timeslot(
        db: &DB,
        vid: VillageKey,
        unit_count: usize,
        mut earliest: NaiveDateTime,
    ) -> NaiveDateTime {
        // TODO (Optimization): These are potentially many DB queries

        // Query returns attacks sorted by arrival date
        let already_running_attacks = db.attacks(vid, None);
        let duration = Self::attack_duration(unit_count);
        let mut i = 0;
        let len = already_running_attacks.len();
        while i < len {
            let atk = &already_running_attacks[i];
            let n = db.attack_hobos(atk.key()).len();
            let d = Self::attack_duration(n);
            if atk.arrival + d <= earliest {
                // No conflict with i, i is earlier than new attack
                i += 1;
            } else if atk.arrival < earliest + duration {
                // Conflict with i, need to delay new attack to be after i
                i += 1;
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
        let seconds = (units + 1) * 5 / 2;
        chrono::Duration::seconds(seconds as i64)
    }
}
