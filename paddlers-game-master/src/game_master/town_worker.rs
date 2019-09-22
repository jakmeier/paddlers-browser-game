use super::event_queue::*;
use super::event::*;
use crate::db::*;
use actix::prelude::*;
use chrono::prelude::*;
use paddlers_shared_lib::sql_db::sql::GameDB;

/// Actor for moving around workers inside the town
pub struct TownWorker {
    dbpool: Pool,
    event_queue: EventQueue,
}

impl TownWorker {
    pub fn new(dbpool: Pool) -> Self { 
        TownWorker {
            dbpool: dbpool,
            event_queue: EventQueue::new(),
        }
        .with_filled_event_queue()
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }
    fn work(&mut self, ctx: &mut Context<Self>) {
        while let Some(event) = self.event_queue.poll_event() {
           let res = event.run(&self.db());
           if let Some((next_event, time)) = res {
               self.event_queue.add_event(next_event, time);
           }
        }
        ctx.run_later(std::time::Duration::from_millis(100), Self::work);
    }
    fn with_filled_event_queue(mut self) -> Self {
        let db = self.db();
        // TODO [Village ids]
        for village_id in &[1] {
            for unit in db.workers(*village_id) {
                if let Some((event, time)) = Event::load_next_worker_task(&db, unit.id) {
                    self.event_queue.add_event(event, time);
                }
            }
        }
        self
    }
}

impl Actor for TownWorker {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Town Worker started");
       self.work(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Town Worker stopped");
    }
}

#[derive(Debug)]
pub struct TownWorkerEventMsg(pub Event, pub DateTime<Utc>);

impl Message for TownWorkerEventMsg {
    type Result = ();
}
impl Handler<TownWorkerEventMsg> for TownWorker {
    type Result = ();
    fn handle(&mut self, msg: TownWorkerEventMsg, _ctx: &mut Context<Self>) {
        self.event_queue.add_event(msg.0, msg.1);
    }
}

