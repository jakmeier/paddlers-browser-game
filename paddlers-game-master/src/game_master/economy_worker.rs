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
        let village_id = 1; // TODO village id

        let n = db.workers_with_job(village_id, &[TaskType::GatherSticks]).len();
        let new_sticks = n as i64;
        db.add_resource(ResourceType::Sticks, TEST_VILLAGE_ID, new_sticks).expect("Adding resources");

        let n = db.workers_with_job(village_id, &[TaskType::ChopTree]).len();
        let new_logs = n as i64;
        db.add_resource(ResourceType::Logs, TEST_VILLAGE_ID, new_logs).expect("Adding logs");

         // TODO: Exact econ calculations: Extra DB table for timestamp of last update instead of wait constant time
        ctx.run_later(std::time::Duration::from_millis(5000), Self::work);

        let workers = db.workers(village_id);
        let now = chrono::Utc::now().naive_utc();
        for w in workers {
            if let Some(last_update) = db.last_update(w.key(), WorkerFlagType::ManaRegeneration) {
                let mana_regen = hero_mana_regeneration_per_hour();
                let interval_ms = 3_600_000 / mana_regen as i64;
                let new_mana = (now - last_update).num_milliseconds() / interval_ms;
                if new_mana > 0 {
                    let new_time = last_update + chrono::Duration::milliseconds(interval_ms * new_mana);
                    db.update_worker_flag_timestamp(
                        w.key(), 
                        WorkerFlagType::ManaRegeneration,
                        new_time,
                    );
                    db.add_worker_mana(w.key(), new_mana as i32, hero_max_mana());
                }
            }
        }
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