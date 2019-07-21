use std::collections::VecDeque;
use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::Timestamp;
use crate::game::{
    town::{Town, TileIndex},
};
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::tasks::*;


#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Worker {
    tasks: VecDeque<WorkerTask>,
}

pub struct WorkerTask {
    pub task_type: TaskType, 
    pub position: TileIndex,
    pub start_time: Timestamp,
}


impl Worker {
    pub fn walk(&mut self, from: TileIndex, to: TileIndex, town: &Town, netid: i64,) -> Result<TaskList, String> {
        if let Some((path, _dist)) = town.shortest_path(from, to) {
            let msg = TaskList {
                unit_id: netid,
                tasks: path_to_raw_tasks(&path, from),
            };
            Ok(msg)
        }
        else {
            Err(format!("Cannot walk from {:?} to {:?}", from, to))
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
}

fn path_to_raw_tasks(path: &[TileIndex], from: TileIndex) -> Vec<RawTask> {
    let mut tasks = vec![];
    let mut current_direction = Vector::new(0,0);
    let mut current = from;
    for next in path {
        let next_direction = direction_vector(current, *next);
        if next_direction != current_direction && current_direction != Vector::new(0,0) {
            tasks.push(
                RawTask {
                    task_type: TaskType::Walk,
                    x: current.0,
                    y: current.1,
                }
            )
        }
        current = *next;
        current_direction = next_direction;
    }
    tasks.push(
        RawTask {
            task_type: TaskType::Walk, 
            x: current.0,
            y: current.1,
        }
    );
    tasks.push(
        RawTask {
            task_type: TaskType::Idle, 
            x: current.0,
            y: current.1,
        }
    );
    tasks
}

fn direction_vector(a: TileIndex, b: TileIndex) -> Vector {
    let a = Vector::new(a.0 as u32, a.1 as u32);
    let b = Vector::new(b.0 as u32, b.1 as u32);
    a - b
}