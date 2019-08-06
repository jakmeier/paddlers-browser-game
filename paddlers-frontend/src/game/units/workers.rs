use std::collections::VecDeque;
use quicksilver::geom::*;
use specs::prelude::*;
use crate::prelude::*;
use crate::game::{
    town::{Town, TileIndex},
    movement::Position,
    components::*,
};
use crate::gui::render::Renderable;
use crate::gui::z::*;
use crate::net::game_master_api::RestApiState;
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
    pub fn task_on_right_click<'a>(&mut self, from: TileIndex, click: &Vector, town: &Town, containers: &ReadStorage<'a, EntityContainer>) -> PadlResult<Option<TaskList>> {
        let destination = town.tile(*click);
        if let Some(job) = self.task_at_position(town, destination) {
            // Check with stateful tiles for validity
            if let Some(tile_state) = town.tile_state(destination) {
                match job {
                    TaskType::GatherSticks
                    | TaskType::ChopTree
                        => {
                        if let Some(container) = containers.get(tile_state.entity) {
                            if !container.can_add_entity() {
                                return PadlErrorCode::BuildingFull(town.building_type(destination)?).usr();
                            }
                        }
                        else {
                            return PadlErrorCode::DevMsg("Cannot gather resources here.").usr();
                        }
                    }
                    TaskType::Idle | TaskType::Walk => {},
                    TaskType::Defend  => { panic!("NIY") },
                }

            }
            // Check global supply constraints
            let forest_requirement = job.required_forest_size();
            if town.forest_size_free() < forest_requirement {
                return PadlErrorCode::ForestTooSmall(forest_requirement - town.forest_size_free()).usr();
            }
            if let Some((path, _dist)) = town.shortest_path(from, destination) {
                let mut tasks = raw_walk_tasks(&path, from);
                tasks.push( RawTask::new(job, destination) );
                let msg = TaskList {
                    unit_id: self.netid,
                    tasks: tasks,
                };
                Ok(Some(msg))
            } else {
                PadlErrorCode::PathBlocked.usr()
            }
        } else {
            // Nothing to do here
            Ok(None)
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
    town: &mut Write<'a, Town>,
    lazy: &Read<'a, LazyUpdate>,
    rend: &ReadStorage<'a, Renderable>,
    worker_e: Entity, 
    building_pos: TileIndex,
){
    let renderable = rend.get(worker_e).unwrap();
    let tile_state = (*town).tile_state(building_pos).unwrap();
    let c = containers.get_mut(tile_state.entity).unwrap();
    c.add_entity_unchecked(worker_e, &renderable);
    town.add_entity_to_building(&building_pos).expect("Task has conflict");
    town.add_stateful_task(c.task).expect("Task has conflict in town state");
    lazy.remove::<Position>(worker_e);
}

pub fn move_worker_out_of_building<'a>(
    town: &mut Write<'a, Town>,
    worker_e: Entity,
    task: TaskType,
    workers: &mut WriteStorage<'a, Worker>,
    tile: TileIndex,
    size: Vector,
    lazy: &Read<'a, LazyUpdate>,
    rest: &mut WriteExpect<'a, RestApiState>,
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
    town.remove_entity_from_building(&tile).unwrap();
    town.remove_stateful_task(task).expect("Task has conflict in town state");
}