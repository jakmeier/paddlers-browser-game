use std::collections::VecDeque;
use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::Timestamp;
use crate::game::{
    input::Clickable,
    movement::{Position, Moving},
    town::{Town, TileIndex},
};
use paddlers_shared_lib::models::*;


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
    pub fn walk(&mut self, from: TileIndex, to: TileIndex, town: &Town) {
        if let Some((path, _dist)) = town.shortest_path(from, to) {
            let mut current_direction = Vector::new(0,0);
            let mut current = from;
            for next in path {
                let next_direction = direction_vector(current, next);
                if next_direction != current_direction {
                    // TODO: Prepare network package instead
                    self.tasks.push_back(
                        WorkerTask {
                            task_type: TaskType::Walk, 
                            position: current,
                            start_time: crate::wasm_setup::local_now(), //XXX
                        }
                    )
                }
                current = next;
                current_direction = next_direction;
            }
            self.tasks.push_back(
                WorkerTask {
                    task_type: TaskType::Walk, 
                    position: to,
                    start_time: crate::wasm_setup::local_now(),//XXXX
                }
            );
            self.tasks.push_back(
                WorkerTask {
                    task_type: TaskType::Idle, 
                    position: to,
                    start_time: crate::wasm_setup::local_now() + 1000.0,//XXXX
                }
            );
        }
        else {
            println!("Cannot walk from {:?} to {:?}", from, to);
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

fn direction_vector(a: TileIndex, b: TileIndex) -> Vector {
    let a = Vector::new(a.0 as u32, a.1 as u32);
    let b = Vector::new(b.0 as u32, b.1 as u32);
    a - b
}