//! Spawns random attacks on villages

use crate::db::*;
use crate::game_master::attack_funnel::{AttackFunnel, PlannedAttack};
use actix::prelude::*;
use futures::future::join_all;
use paddlers_shared_lib::prelude::*;
use rand::Rng;

pub struct AttackSpawner {
    dbpool: Pool,
    db_actor: Addr<DbActor>,
    attack_funnel_actor: Addr<AttackFunnel>,
}

impl Actor for AttackSpawner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Attack Spawner is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Attack Spawner is stopped");
    }
}

pub(super) struct AttackTarget(pub VillageKey);
impl Message for AttackTarget {
    type Result = ();
}

impl Handler<AttackTarget> for AttackSpawner {
    type Result = ();

    fn handle(&mut self, msg: AttackTarget, _ctx: &mut Context<Self>) -> Self::Result {
        self.spawn_random_attack(msg.0);
    }
}

impl AttackSpawner {
    pub fn new(
        dbpool: Pool,
        db_actor: Addr<DbActor>,
        attack_funnel_actor: Addr<AttackFunnel>,
    ) -> Self {
        AttackSpawner {
            dbpool,
            db_actor,
            attack_funnel_actor,
        }
    }
    fn db(&self) -> DB {
        (&self.dbpool).into()
    }

    fn spawn_random_attack(&self, village: VillageKey) {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(2, 5);

        let futures: Vec<Request<DbActor, NewHoboMessage>> = (0..n)
            .map(|_| {
                let hobo = NewHobo {
                    color: Some(Self::gen_color(&mut rng)),
                    hp: rng.gen_range(1, 10),
                    speed: 0.1,
                    home: village.num(), // TODO: anarchists home
                };
                let msg = NewHoboMessage(hobo);
                self.db_actor.send(msg)
            })
            .collect();

        let attack_funnel = self.attack_funnel_actor.clone();
        let pool = self.dbpool.clone();
        Arbiter::spawn(
            join_all(futures)
                .map_err(|e| eprintln!("Attack spawn failed: {:?}", e))
                .map(move |hobos| {
                    let hobos = hobos.into_iter().map(|h| h.0).collect();
                    let db : DB = (&pool).into();

                    let pa = PlannedAttack {
                        origin_village: None,
                        destination_village: db.village(village).unwrap(),
                        hobos: hobos,
                    };
                    attack_funnel.try_send(pa).expect("Spawning attack failed");
                }),
        );
    }

    fn gen_color<R>(rng: &mut R) -> UnitColor
    where
        R: Rng,
    {
        match rng.gen_range(0, 100) {
            0..85 => UnitColor::Yellow,
            85..99 => UnitColor::Camo,
            99 => UnitColor::White,
            _ => panic!("RNG bug?"),
        }
    }
}
