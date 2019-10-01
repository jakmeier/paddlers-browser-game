use crate::db::*;
use actix::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::game_mechanics::worker::*;

/// Actor for calculating gathered regular events on workers (resource collection, mana regeneration)
pub struct EconomyWorker {
    dbpool: Pool,
}

impl EconomyWorker {
    pub fn new(dbpool: Pool) -> Self { 
        EconomyWorker {
            dbpool: dbpool,
        }
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }
    fn work(&mut self, ctx: &mut Context<Self>) {
        let db = &self.db();
        let village_id = TEST_VILLAGE_ID; // TODO village id

        let workers = db.workers(village_id.num());
        let now = chrono::Utc::now().naive_utc();
        for w in workers {
            for flag in db.worker_flags(w.key()) {
                match flag.flag_type {
                    WorkerFlagType::ManaRegeneration => {
                        let mana_regen = hero_mana_regeneration_per_hour();
                        let interval_ms = 3_600_000 / mana_regen as i64;
                        let new_mana = (now - flag.last_update).num_milliseconds() / interval_ms;
                        if new_mana > 0 {
                            let new_time = flag.last_update + chrono::Duration::milliseconds(interval_ms * new_mana);
                            db.update_worker_flag_timestamp(
                                w.key(), 
                                WorkerFlagType::ManaRegeneration,
                                new_time,
                            );
                            db.add_worker_mana(w.key(), new_mana as i32, hero_max_mana());
                        }
                    },
                    WorkerFlagType::Work => {
                        let task = db.current_task(w.key()).expect("Must have a job");
                        if let Some((res,rate)) = hero_resource_collection_per_hour(task.task_type) {
                            let interval_ms = 3_600_000 / rate as i64;
                            let n = (now - flag.last_update).num_milliseconds() / interval_ms;
                            if n > 0 {
                                let new_time = flag.last_update + chrono::Duration::milliseconds(interval_ms * n);
                                db.update_worker_flag_timestamp(
                                    w.key(), 
                                    WorkerFlagType::Work,
                                    new_time,
                                );
                                db.add_resource(res, village_id, n).expect("Adding resources");
                            }
                        }
                    }
                }
            }
        } 
        
        ctx.run_later(std::time::Duration::from_millis(5000), Self::work);
    }
}

impl Actor for EconomyWorker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
    //    println!("Economy Worker started");
       self.work(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
    //    println!("Economy Worker stopped");
    }
}