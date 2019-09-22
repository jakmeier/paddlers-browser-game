use specs::prelude::*;
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::api::tasks::*;
use crate::prelude::*;
use super::*;
use crate::game::components::EntityContainer;

impl Town {
    pub fn build_task_chain(&self, from: TileIndex, destination: TileIndex, job: TaskType) -> PadlResult<Vec<RawTask>>{
        if let Some((path, _dist)) = self.shortest_path(from, destination) {
            let mut tasks = raw_walk_tasks(&path, from);
            tasks.push( RawTask::new(job, destination) );
            Ok(tasks)
        } else {
            PadlErrorCode::PathBlocked.usr()
        }
    }
    pub fn check_task_constraints<'a>(&self, job: TaskType, destination: TileIndex, containers: &WriteStorage<'a, EntityContainer>) -> PadlResult<()> {
        if let Some(tile_state) = self.tile_state(destination) {
            match job {
                TaskType::GatherSticks
                | TaskType::ChopTree
                    => {
                    if let Some(container) = containers.get(tile_state.entity) {
                        if !container.can_add_entity() {
                            return PadlErrorCode::BuildingFull(Some(self.building_type(destination)?)).usr();
                        }
                    }
                    else {
                        return PadlErrorCode::DevMsg("Cannot gather resources here.").usr();
                    }
                }
                TaskType::Idle | TaskType::Walk => {},
                // TaskType::WelcomeAbility => {},
                TaskType::Defend  => { panic!("NIY") },
            }

        }
        // Check global supply constraints
        let forest_requirement = job.required_forest_size();
        if self.forest_size_free() < forest_requirement {
            return PadlErrorCode::ForestTooSmall(forest_requirement - self.forest_size_free()).usr();
        }
        Ok(())
    }


    pub fn available_tasks(&self, i: TileIndex) -> Vec<TaskType> {
        match self.map[i] {
            TileType::BUILDING(b) => {
                match b {
                    BuildingType::BundlingStation 
                        => vec![TaskType::GatherSticks],
                    BuildingType::SawMill 
                        => vec![TaskType::ChopTree],
                    _ => vec![],
                }
            }
            TileType::EMPTY => {
                vec![TaskType::Idle]
            }
            TileType::LANE => {
                // TODO: Check for welcoming ability
                vec![]
            }
        }
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