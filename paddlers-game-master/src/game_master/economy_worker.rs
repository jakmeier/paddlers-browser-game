use crate::db::*;
use actix::prelude::*;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::sql_db::sql::GameDB;

/// Actor for calculating gathered resources by workers
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

        let n = db.units_with_job(village_id, &[TaskType::GatherSticks]).len();
        let new_sticks = n as i64;
        db.add_resource(ResourceType::Sticks, new_sticks).expect("Adding resources");

        let n = db.units_with_job(village_id, &[TaskType::ChopTree]).len();
        let new_logs = n as i64;
        db.add_resource(ResourceType::Logs, new_logs).expect("Adding logs");

         // TODO: Exact econ calculations: Extra DB table for timestamp of last update instead of wait constant time
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