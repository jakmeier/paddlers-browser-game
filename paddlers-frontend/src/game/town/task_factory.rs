use super::*;
use crate::game::components::{EntityContainer, Mana};
use crate::prelude::*;
use paddlers_shared_lib::api::tasks::*;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

/// Used to describe a new worker-task that has not been processed or checked yet
pub type NewTaskDescriptor = (TaskType, Option<PadlId>);

impl Town {
    pub fn build_task_chain(
        &self,
        from: TileIndex,
        destination: TileIndex,
        job: &NewTaskDescriptor,
    ) -> PadlResult<Vec<RawTask>> {
        if let Some((path, _dist)) = self.shortest_path(from, destination) {
            let mut tasks = raw_walk_tasks(&path, from);
            tasks.extend(raw_job_execution_tasks(job, destination));
            Ok(tasks)
        } else {
            PadlErrorCode::PathBlocked.usr()
        }
    }
    pub fn check_task_constraints<'a>(
        &self,
        job: NewTaskDescriptor,
        destination: TileIndex,
        containers: &WriteStorage<'a, EntityContainer>,
        mana: Option<&Mana>,
    ) -> PadlResult<()> {
        match job.0 {
            TaskType::GatherSticks | TaskType::ChopTree => {
                if let Some(tile_state) = self.tile_state(destination) {
                    if let Some(container) = containers.get(tile_state.entity) {
                        if !container.can_add_entity() {
                            return PadlErrorCode::BuildingFull(Some(
                                self.building_type(destination)?,
                            ))
                            .usr();
                        }
                    } else {
                        return PadlErrorCode::DevMsg("Cannot gather resources here.").usr();
                    }
                }
            }
            TaskType::WelcomeAbility => {
                if mana.map(|o| o.mana).unwrap_or(0) < AbilityType::Welcome.mana_cost() {
                    return PadlErrorCode::NotEnoughMana.usr();
                }
            }
            TaskType::Defend => panic!("NIY"),
            TaskType::Idle | TaskType::CollectReward | TaskType::Walk => {}
        }

        // Check global supply constraints
        let forest_requirement = job.0.required_forest_size();
        if self.forest_size_free() < forest_requirement {
            return PadlErrorCode::ForestTooSmall(forest_requirement - self.forest_size_free())
                .usr();
        }
        Ok(())
    }

    pub fn available_tasks(&self, i: TileIndex) -> Vec<TaskType> {
        match self.map[i] {
            TileType::BUILDING(b) => match b {
                BuildingType::BundlingStation => vec![TaskType::GatherSticks],
                BuildingType::SawMill => vec![TaskType::ChopTree],
                BuildingType::PresentA | BuildingType::PresentB => vec![TaskType::CollectReward],
                _ => vec![],
            },
            TileType::EMPTY => vec![TaskType::Idle],
            TileType::LANE => {
                // TODO: Check for welcoming ability
                vec![]
            }
        }
    }
}

fn raw_walk_tasks(path: &[TileIndex], from: TileIndex) -> Vec<RawTask> {
    let mut tasks = vec![];
    let mut current_direction = Vector::new(0, 0);
    let mut current = from;
    for next in path {
        let next_direction = direction_vector(current, *next);
        if next_direction != current_direction && current_direction != Vector::new(0, 0) {
            tasks.push(RawTask::new(TaskType::Walk, current));
        }
        current = *next;
        current_direction = next_direction;
    }
    tasks.push(RawTask::new(TaskType::Walk, current));
    tasks
}

fn raw_job_execution_tasks(job: &NewTaskDescriptor, place: TileIndex) -> Vec<RawTask> {
    let actual_task = RawTask::new_with_target(*job, place);

    let mut tasks = vec![actual_task];

    // tasks required afterwards
    match job.0 {
        TaskType::ChopTree | TaskType::GatherSticks | TaskType::Idle | TaskType::Walk => {
            // NOP
        }
        TaskType::CollectReward | TaskType::WelcomeAbility => {
            tasks.push(RawTask::new(TaskType::Idle, place));
        }
        TaskType::Defend => {
            panic!("Not implemented");
        }
    }

    tasks
}

fn direction_vector(a: TileIndex, b: TileIndex) -> Vector {
    let a = Vector::new(a.0 as u32, a.1 as u32);
    let b = Vector::new(b.0 as u32, b.1 as u32);
    a - b
}
