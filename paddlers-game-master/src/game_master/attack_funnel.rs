//! Funnels planned attacks
//! 
//! All attacks on villages should go through this funnel.
//! This module should then guarantee that no two attacks reach a town at the same time
//! and that no hobo is involved in more than one attack at the time.

use actix::prelude::*;
use paddlers_shared_lib::prelude::*;
use crate::db::*;

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
    pub origin_village: Option<VillageKey>,
    pub destination_village: VillageKey,
    pub hobos: Vec<HoboKey>,
}
impl Message for PlannedAttack {
    type Result = ();
}


impl Handler<PlannedAttack> for AttackFunnel {
    type Result = ();

    fn handle(&mut self, msg: PlannedAttack, _ctx: &mut Context<Self>) -> Self::Result {
        // TODO: "now" should be next available time slot for destination village
        let now = chrono::Utc::now().naive_utc();
        use std::ops::Add;
        // TODO: Arrival should be depending on distance
        let arrival = now.add(chrono::Duration::seconds(15));
        let attack = NewAttack {
            departure: now,
            arrival: arrival,
            origin_village_id: msg.origin_village.map(|k|k.num()),
            destination_village_id: msg.destination_village.num(),
        };
        // TODO: Somehow efficiently check that hobos are not attacking already (and from this village?)
        let hobos = msg.hobos;

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
}