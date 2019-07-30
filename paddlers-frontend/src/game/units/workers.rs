use std::collections::VecDeque;
use quicksilver::geom::*;
use specs::prelude::*;
use crate::Timestamp;
use crate::game::{
    town::{Town, TileIndex},
    movement::Position,
    components::*,
};
use crate::gui::render::Renderable;
use crate::gui::z::*;
use crate::net::game_master_api::RestApiState;
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::tasks::*;


#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct Worker {
    pub tasks: VecDeque<WorkerTask>,
    pub netid: i64,
}

#[derive(Debug)]
pub struct WorkerTask {
    pub task_type: TaskType, 
    pub position: TileIndex,
    pub start_time: Timestamp,
}

impl Worker {
    pub fn task_on_right_click(&mut self, from: TileIndex, click: &Vector, town: &Town) -> Result<TaskList, String> {
        let destination = town.tile(*click);
        if let Some(job) = self.task_at_position(town, destination) {
            if let Some((path, _dist)) = town.shortest_path(from, destination) {
                let mut tasks = raw_walk_tasks(&path, from);
                tasks.push( RawTask::new(job, destination) );
                let msg = TaskList {
                    unit_id: self.netid,
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
    fn go_idle(&mut self, idx: TileIndex) -> Result<TaskList, String> {
        let tasks = vec![
            RawTask::new(TaskType::Idle, idx)
        ];
        Ok( TaskList {
            unit_id: self.netid,
            tasks: tasks,
        })
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

pub fn move_worker_into_building<'a>(
    containers: &mut WriteStorage<'a, EntityContainer>, 
    positions: &mut WriteStorage<'a, Position>, 
    lazy: &Read<'a, LazyUpdate>,
    rend: &ReadStorage<'a, Renderable>,
    worker_e: Entity, 
    building_pos: Vector,
){
    let renderable = rend.get(worker_e).unwrap();
    for (p, c) in (positions, containers).join() {
         if building_pos.overlaps_rectangle(&p.area) {
            c.add_entity(worker_e, &renderable);
            break;
        }
    }
    lazy.remove::<Position>(worker_e);
}

pub fn move_worker_out_of_building<'a>(
    worker_e: Entity,
    workers: &mut WriteStorage<'a, Worker>,
    tile: TileIndex,
    size: Vector,
    lazy: &Read<'a, LazyUpdate>,
    rest: &mut Write<'a, RestApiState>,

) {
    let worker = workers.get_mut(worker_e).unwrap();
    let http_msg = worker.go_idle(tile);
    match http_msg {
        Ok(msg) => {
            rest.http_overwrite_tasks(msg);
        }
        Err(e) => {
            println!("Failure on moving out of building: {}", e);
        }
    }
    lazy.insert(worker_e, 
        Position::new(
            (0.0,0.0), // the MoveSystem will overwrite this before first use
            size, 
            Z_UNITS
        )
    );
}