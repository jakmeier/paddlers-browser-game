use std::collections::VecDeque;
use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::Timestamp;
use crate::game::{
    town::{Town, TileIndex},
};
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::tasks::*;


#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct Worker {
    pub tasks: VecDeque<WorkerTask>,
}

#[derive(Debug)]
pub struct WorkerTask {
    pub task_type: TaskType, 
    pub position: TileIndex,
    pub start_time: Timestamp,
}

impl Worker {
    pub fn task_on_right_click(&mut self, from: TileIndex, click: &Vector, town: &Town, netid: i64,) -> Result<TaskList, String> {
        let destination = town.tile(*click);
        if let Some(job) = self.task_at_position(town, destination) {
            if let Some((path, _dist)) = town.shortest_path(from, destination) {
                let mut tasks = raw_walk_tasks(&path, from);
                tasks.push( RawTask::new(job, destination) );
                let msg = TaskList {
                    unit_id: netid,
                    tasks: tasks,
                };
                Ok(msg)
            } else {
                Err(format!("Cannot walk from {:?} to {:?}", from, destination))
            }
        } else {
            Err(format!("No job to do at {:?}", destination))
        }
    }

    pub fn poll(&mut self, t: Timestamp) -> Option<WorkerTask> {
        if let Some(next_task) = self.tasks.front() {
            if next_task.start_time < t {
                return self.tasks.pop_front();
            }
        }
        None
    }
    fn task_at_position(&self, town: &Town, i: TileIndex) -> Option<TaskType> {
        let jobs = town.available_tasks(i);
        jobs.into_iter()
            // .filter(
            //     || TODO
            // )
            .next()
    }
}

fn raw_walk_tasks(path: &[TileIndex], from: TileIndex) -> Vec<RawTask> {
    let mut tasks = vec![];
    let mut current_direction = Vector::new(0,0);
    let mut current = from;
    for next in path {
        let next_direction = direction_vector(current, *next);
        if next_direction != current_direction && current_direction != Vector::new(0,0) {
            tasks.push( RawTask::new(TaskType::Walk, current) );
        }
        current = *next;
        current_direction = next_direction;
    }
    tasks.push( RawTask::new(TaskType::Walk, current) );
    tasks
}

fn direction_vector(a: TileIndex, b: TileIndex) -> Vector {
    let a = Vector::new(a.0 as u32, a.1 as u32);
    let b = Vector::new(b.0 as u32, b.1 as u32);
    a - b
}