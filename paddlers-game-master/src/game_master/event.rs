use crate::db::*;
use crate::worker_actions::finish_task;
use chrono::prelude::*;
use paddlers_shared_lib::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Event {
    WorkerTask { task_id: i64 },
}

impl Event {
    pub(super) fn run(&self, db: &DB) -> Option<(Event, DateTime<Utc>)> {
        match self {
            Event::WorkerTask { task_id } => {
                let res = finish_task(db, *task_id, None, None);
                if let Err(e) = res {
                    println!("Task execution failed: {}", e);
                    None
                } else {
                    res.unwrap()
                }
            }
        }
    }
    pub(crate) fn load_next_worker_task(
        db: &DB,
        worker_id: WorkerKey,
    ) -> Option<(Self, DateTime<Utc>)> {
        let (current, next) = db.current_and_next_task(worker_id);
        let current_task = current.expect("Units must always have a task");
        next.map(|next_task| {
            (
                Event::WorkerTask {
                    task_id: current_task.id,
                },
                Utc.from_utc_datetime(&next_task.start_time),
            )
        })
    }
}
