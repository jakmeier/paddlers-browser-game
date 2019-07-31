use crate::db::*;
use chrono::prelude::*;
use crate::worker_actions::finish_task;
use paddlers_shared_lib::sql_db::sql::GameDB;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Event {
    UnitTask{ task_id: i64 },
}

impl Event {
    pub (super) fn run(&self, db: &DB) -> Option<(Event, DateTime<Utc>)> {
        match self {
            Event::UnitTask{ task_id } => {
                finish_task(db, *task_id, None, None).expect("Task execution failed.")
            }
        }
    }
    pub (crate) fn load_next_unit_task(db: &DB, unit_id: i64) -> Option<(Self, DateTime<Utc>)> {
        let (current, next) = db.current_and_next_task(unit_id);
        let current_task = current.expect("Units must always have a task");
        next.map( |next_task|
            (
                Event::UnitTask{ task_id: current_task.id },
                Utc.from_utc_datetime(&next_task.start_time)
            )
        )
    }
}