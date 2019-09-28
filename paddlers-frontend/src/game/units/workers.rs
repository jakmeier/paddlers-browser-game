use std::collections::VecDeque;
use quicksilver::geom::*;
use specs::prelude::*;
use crate::prelude::*;
use crate::game::{
    town::{Town, TileIndex, task_factory::NewTaskDescriptor},
    movement::{Position},
    components::*,
};
use crate::gui::render::Renderable;
use crate::gui::z::*;
use crate::net::game_master_api::RestApiState;
use crate::logging::ErrorQueue;
use paddlers_shared_lib::api::tasks::*;

#[derive(Default, Component, Debug)]
#[storage(HashMapStorage)]
pub struct Worker {
    pub tasks: VecDeque<WorkerTask>,
    pub netid: i64,
}

#[derive(Debug)]
pub struct WorkerTask {
    pub task_type: TaskType, 
    pub position: TileIndex,
    pub start_time: Timestamp,
    pub target: Option<Entity>,
}

impl Worker {
    /// Worker is ordered by the player to perform a job at a position
    /// How to get there and if this is possible has yet to be checked.
    pub fn new_order<'a>(
        &mut self,
        entity: Entity,
        start: TileIndex,
        job: NewTaskDescriptor,
        destination: TileIndex,
        town: &Town,
        rest: &mut RestApiState,
        errq: &mut ErrorQueue,
        containers: &mut WriteStorage<'a, EntityContainer>,
        mana: &ReadStorage<'a, Mana>,
    ) {
        let msg = self.try_create_task_list(entity, start, destination, job, &town, containers, mana);
        match msg {
            Ok(msg) => {
                rest.http_overwrite_tasks(msg)
                    .unwrap_or_else(|e| errq.push(e));
            }
            Err(e) => {
                errq.push(e);
            }
        }
    }

    /// Create a list of tasks that walk a worker to a place and let's it perform a job there.
    /// The returned format can be understood by the backend interface.
    /// Returns an error if the job cannot be done by this worker at the desired position.
    pub fn try_create_task_list<'a>(
        &mut self,
        entity: Entity,
        from: TileIndex,
        destination: TileIndex,
        job: NewTaskDescriptor,
        town: &Town,
        containers: &mut WriteStorage<'a, EntityContainer>,
        mana: &ReadStorage<'a, Mana>,
) -> PadlResult<TaskList> {
        let mana = mana.get(entity);
        town.check_task_constraints(job, destination, containers, mana)?;
        let tasks = town.build_task_chain(from, destination, &job)?;
        let msg = TaskList {
            worker_id: self.netid,
            tasks: tasks,
        };
        Ok(msg)
    }
    
    /// Finds the default-task that is performed on a right click in the town area
    pub fn task_on_right_click<'a>(&mut self, click: &Vector, town: &Town) -> Option<(TaskType, TileIndex)> {
        let destination = town.tile(*click); // TODO: destination is not always where it has been clicked
        let job = town.available_tasks(destination)
            .into_iter()
            // .filter(
            //     || TODO
            // )
            .next()?;
        Some((job, destination))
    }
    
    fn go_idle(&mut self, idx: TileIndex) -> Result<TaskList, String> {
        let tasks = vec![
            RawTask::new(TaskType::Idle, idx)
        ];
        Ok( TaskList {
            worker_id: self.netid,
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
}

pub fn move_worker_into_building<'a>(
    containers: &mut WriteStorage<'a, EntityContainer>, 
    ui_menus: &mut WriteStorage<'a, UiMenu>, 
    town: &mut Write<'a, Town>,
    lazy: &Read<'a, LazyUpdate>,
    rend: &ReadStorage<'a, Renderable>,
    worker_e: Entity, 
    building_pos: TileIndex,
){
    let renderable = rend.get(worker_e).unwrap();
    let tile_state = (*town).tile_state(building_pos).unwrap();
    let c = containers.get_mut(tile_state.entity).unwrap();
    let mut ui_menu = ui_menus.get_mut(tile_state.entity).unwrap();
    c.add_entity_unchecked(worker_e, &renderable, &mut ui_menu);
    town.add_entity_to_building(&building_pos).expect("Task has conflict");
    town.add_stateful_task(c.task).expect("Task has conflict in town state");
    lazy.remove::<Position>(worker_e);
}

pub fn move_worker_out_of_building<'a>(
    town: &mut Town,
    worker_e: Entity,
    task: TaskType,
    workers: &mut WriteStorage<'a, Worker>,
    tile: TileIndex,
    size: Vector,
    lazy: &Read<'a, LazyUpdate>,
    rest: &mut WriteExpect<'a, RestApiState>,
) -> PadlResult<()>
{
    let worker = workers.get_mut(worker_e).unwrap();
    let http_msg = worker.go_idle(tile);
    match http_msg {
        Ok(msg) => {
            rest.http_overwrite_tasks(msg)?;
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
    Ok(())
}