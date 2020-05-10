use crate::db::*;
use crate::worker_actions::finish_task;
use chrono::prelude::*;
use paddlers_shared_lib::game_mechanics::town::MAX_VISITOR_QUEUE;
use paddlers_shared_lib::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
/// For actions that must be performed at a later point in time.
/// These events can be queued up in the `EventQueue`
pub enum Event {
    WorkerTask { task_id: TaskKey },
    CheckRestingVisitors { village_id: VillageKey },
}

impl Event {
    pub(super) fn run(&self, db: &DB) -> Option<(Event, DateTime<Utc>)> {
        match self {
            Self::WorkerTask { task_id } => {
                let res = finish_task(db, *task_id, None, None);
                if let Err(e) = res {
                    println!("Task execution failed: {}", e);
                    None
                } else {
                    res.unwrap()
                }
            }
            Self::CheckRestingVisitors { village_id } => {
                // Release all visitors that are queued beyond the limit
                let visitors = db.resting_visitors(*village_id);
                if visitors.len() > MAX_VISITOR_QUEUE {
                    for (hobo, attack_id) in &visitors[MAX_VISITOR_QUEUE..] {
                        db.release_resting_visitor(hobo.key(), *attack_id)
                    }
                }
                None
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
                    task_id: current_task.key(),
                },
                Utc.from_utc_datetime(&next_task.start_time),
            )
        })
    }
}
