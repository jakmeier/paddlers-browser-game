use crate::db::*;
use chrono::prelude::*;
use crate::worker_actions::finish_task;
use paddlers_shared_lib::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Event {
    WorkerTask{ task_id: i64 },
}

impl Event {
    pub (super) fn run(&self, db: &DB) -> Option<(Event, DateTime<Utc>)> {
        match self {
            Event::WorkerTask{ task_id } => {
                finish_task(db, *task_id, None, None).expect("Task execution failed.")
            }
        }
    }
    pub (crate) fn load_next_worker_task(db: &DB, worker_id: WorkerKey) -> Option<(Self, DateTime<Utc>)> {
        let (current, next) = db.current_and_next_task(worker_id);
        let current_task = current.expect("Units must always have a task");
        next.map( |next_task|
            (
                Event::WorkerTask{ task_id: current_task.id },
                Utc.from_utc_datetime(&next_task.start_time)
            )
        )
    }
}