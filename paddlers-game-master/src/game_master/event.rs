use crate::db::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Event {
    UnitTask{ task_id: i64 },
}

impl Event {
    pub (super) fn run(&self, db: &DB) {
        match self {
            Event::UnitTask{ task_id } => {
                crate::worker_actions::execute_task(db, *task_id, None, None).expect("Task execution failed.");
            }
        }
    }
}