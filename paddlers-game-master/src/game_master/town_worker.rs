use super::event_queue::*;
use super::event::*;
use crate::db::*;
use actix::prelude::*;
use chrono::prelude::*;


pub struct TownWorker {
    dbpool: Pool,
    event_queue: EventQueue,
}

impl TownWorker {
    pub fn new(dbpool: Pool) -> Self {
        TownWorker {
            dbpool: dbpool,
            event_queue: EventQueue::new(), // TODO: Fill event queue with initial work
        }
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }
    fn work(&mut self, ctx: &mut Context<Self>) {
        while let Some(event) = self.event_queue.poll_event() {
           event.run(&self.db())
        }
        if let Some(t) = self.event_queue.time_of_next_event() {
            let duration = *t - chrono::Utc::now();
            let duration : std::time::Duration = duration.to_std().unwrap();
            ctx.run_later(duration, Self::work);
        }
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

