//! Spawns random attacks on villages

use crate::db::*;
use crate::game_master::attack_funnel::{AttackFunnel, PlannedAttack};
use actix::prelude::*;
use futures::future::join_all;
use paddlers_shared_lib::specification_types::{HoboLevel, HoboType};
use paddlers_shared_lib::{prelude::*, specification_types::VisitorDefinition};
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

/// Randomized attack from no specific origin
pub(super) struct SendAnarchistAttack {
    pub village: VillageKey,
    pub level: HoboLevel,
}

impl Message for SendAnarchistAttack {
    type Result = ();
}
impl Handler<SendAnarchistAttack> for AttackSpawner {
    type Result = ();

    fn handle(&mut self, msg: SendAnarchistAttack, _ctx: &mut Context<Self>) -> Self::Result {
        self.spawn_anarchists(msg.village, msg.level);
    }
}

/// Specific attack to be sent without persistent hobos (spawn them on-the-fly, deletion still TODO)
pub(super) struct SendAnonymousAttack {
    pub destination: VillageKey,
    pub origin: VillageKey,
    pub visitors: Vec<VisitorDefinition>,
    pub fixed_travel_time_s: Option<i32>,
}
impl Message for SendAnonymousAttack {
    type Result = ();
}
impl Handler<SendAnonymousAttack> for AttackSpawner {
    type Result = ();

    fn handle(&mut self, msg: SendAnonymousAttack, _ctx: &mut Context<Self>) -> Self::Result {
        self.spawn_anonymous(
            msg.destination,
            msg.origin,
            msg.visitors,
            msg.fixed_travel_time_s,
        );
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

    fn spawn_anarchists(&self, village: VillageKey, level: HoboLevel) {
        // Send a random number of weak and hurried hobos + 1 stronger which will rest until satisfied or replaced
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(2, 4);

        // weak hobos
        let mut hobos: Vec<VisitorDefinition> = (0..n)
            .map(|_| VisitorDefinition {
                typ: HoboType::DefaultRandom,
                level,
                hurried: true,
                hp: None,
            })
            .collect();
        hobos.push(VisitorDefinition {
            typ: HoboType::DefaultRandom,
            level,
            hurried: false,
            hp: None,
        });
        self.spawn_anonymous(
            village, village, /* TODO: pick an anarchist village instead */
            hobos, None,
        );
    }
    fn spawn_anonymous(
        &self,
        destination: VillageKey,
        origin: VillageKey,
        visitors: Vec<VisitorDefinition>,
        fixed_travel_time_s: Option<i32>,
    ) {
        let mut rng = rand::thread_rng();
        let futures: Vec<Request<DbActor, NewHoboMessage>> = visitors
            .into_iter()
            .map(|def| {
                let hp;
                if let Some(fixed_hp) = def.hp {
                    hp = fixed_hp as i64;
                } else if def.hurried {
                    let (min_hp, max_hp) = def.level.hurried_anarchist_hp_range();
                    hp = rng.gen_range(min_hp, max_hp);
                } else {
                    hp = def.level.unhurried_anarchist_hp();
                };
                let color = Self::unit_color(&mut rng, def.typ);
                let hobo = NewHobo {
                    color: Some(color),
                    hp,
                    speed: 0.05,
                    home: origin.num(),
                    hurried: def.hurried,
                    nest: None,
                };
                let msg = NewHoboMessage(hobo);
                self.db_actor.send(msg)
            })
            .collect();

        let attack_funnel = self.attack_funnel_actor.clone();
        let pool = self.dbpool.clone();

        let planned_attack = join_all(futures)
            .and_then(move |hobos| {
                let hobos = hobos.into_iter().map(|h| h.0).collect();
                let db: DB = (&pool).into();

                let pa = PlannedAttack {
                    origin_village: None,
                    destination_village: db.village(destination).unwrap(),
                    hobos: hobos,
                    fixed_travel_time_s,
                    subject_to_visitor_queue_limit: true,
                };
                attack_funnel.send(pa)
            })
            .map_err(|e| eprintln!("Attack spawn failed: {:?}", e));
        Arbiter::spawn(planned_attack);
    }

    fn unit_color<R>(rng: &mut R, hobo_color: HoboType) -> UnitColor
    where
        R: Rng,
    {
        match hobo_color {
            HoboType::DefaultRandom => Self::gen_color(rng),
            HoboType::Yellow => UnitColor::Yellow,
            HoboType::Camo => UnitColor::Camo,
            HoboType::White => UnitColor::White,
            HoboType::Prophet => UnitColor::Prophet,
        }
    }
    fn gen_color<R>(rng: &mut R) -> UnitColor
    where
        R: Rng,
    {
        match rng.gen_range(0, 100) {
            0..85 => UnitColor::Yellow,
            85..99 => UnitColor::Camo,
            99 => UnitColor::White,
            _ => unreachable!(),
        }
    }
}
