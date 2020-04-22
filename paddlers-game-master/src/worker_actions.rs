//! Tasks and task execution of workes
//!
//! Note: This module and submodules will sooner or later need some refactoring.
//! For now, I am still don't really know how I want it to look like.

mod worker_abilities;
mod worker_updates;

use crate::db::DB;
use crate::game_master::event::*;
use crate::game_master::town_worker::*;
use crate::town_view::*;
use actix::prelude::*;
use chrono::offset::TimeZone;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use paddlers_shared_lib::api::tasks::*;
use paddlers_shared_lib::game_mechanics::worker::*;
use paddlers_shared_lib::prelude::*;
use worker_abilities::*;
use worker_updates::MutWorkerDBEntity;

trait WorkerAction {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn task_type(&self) -> &TaskType;
    fn target(&self) -> Option<HoboKey>;
}
pub struct ValidatedTaskList {
    pub new_tasks: Vec<NewTask>,
    pub update_tasks: Vec<Task>,
    pub village_id: VillageKey,
}
pub(crate) fn validate_task_list(
    db: &DB,
    tl: &TaskList,
) -> Result<ValidatedTaskList, Box<dyn std::error::Error>> {
    let worker_id = tl.worker_id;

    // Load relevant data into memory
    let mut worker = db.worker_priv(worker_id).ok_or("Worker does not exist")?;
    let village_id = VillageKey(worker.home);
    let mut town = TownView::load_village(db, village_id);

    // check timing and effect of current task interruption
    let mut current_task = db
        .current_task(worker.key())
        .expect("Must have a current task");
    let mut timestamp =
        interrupt_task(&mut current_task, &worker).ok_or("Cannot interrupt current task.")?;
    worker.x = current_task.x;
    worker.y = current_task.y;

    // iterate tasks and match for task types
    let mut tasks = vec![];

    for task in tl.tasks.iter() {
        // Validate target hobo exists if there is one
        if let Some(target_id) = task.target {
            db.hobo(target_id).ok_or("No such hobo id")?;
        }

        validate_ability(db, task.task_type, worker_id, timestamp)?;

        let new_task = NewTask {
            worker_id: worker_id.num(),
            task_type: task.task_type,
            x: task.x as i32,
            y: task.y as i32,
            start_time: Some(timestamp),
            target_hobo_id: task.target,
        };
        simulate_begin_task(&new_task, &mut town, &mut worker)?;
        let duration = simulate_finish_task(&new_task, &mut town, &mut worker)?;
        tasks.push(new_task);
        timestamp += duration;
    }
    Ok(ValidatedTaskList {
        new_tasks: tasks,
        update_tasks: vec![current_task],
        village_id,
    })
}
pub(crate) fn replace_worker_tasks(
    db: &DB,
    worker: &Addr<TownWorker>,
    worker_id: WorkerKey,
    tasks: &[NewTask],
    village_id: VillageKey,
) {
    db.flush_task_queue(worker_id);
    let _inserted = db.insert_tasks(tasks);
    let current_task =
        execute_worker_tasks(db, worker_id, village_id).expect("Worker has no current task");
    if let Some(next_task) = db.earliest_future_task(worker_id) {
        let event = Event::WorkerTask {
            task_id: current_task.id,
        };
        worker
            .send(TownWorkerEventMsg(
                event,
                Utc.from_utc_datetime(&next_task.start_time),
            ))
            .wait()
            .expect("Send msg to actor");
    }
}

fn interrupt_task(current_task: &mut Task, worker: &Worker) -> Option<NaiveDateTime> {
    match current_task.task_type {
        TaskType::Idle
        | TaskType::ChopTree
        | TaskType::Defend
        | TaskType::GatherSticks
        | TaskType::CollectReward => {
            let now = chrono::Utc::now().naive_utc();
            Some(now)
        }
        TaskType::Walk => {
            let speed = unit_speed_to_worker_tiles_per_second(worker.speed) as f64;
            let time_so_far: Duration = Utc::now().naive_utc() - current_task.start_time;
            let steps = (speed * time_so_far.num_microseconds().unwrap() as f64 / 1_000_000.0)
                .ceil() as i32;
            let total_time = steps as f64 / speed;
            let moment = current_task.start_time
                + chrono::Duration::microseconds((total_time * 1_000_000.0) as i64);
            let dx = current_task.x - worker.x;
            let dy = current_task.y - worker.y;
            let x = if dx == 0 {
                worker.x
            } else if dx < 0 {
                worker.x - steps
            } else {
                worker.x + steps
            };
            let y = if dy == 0 {
                worker.y
            } else if dy < 0 {
                worker.y - steps
            } else {
                worker.y + steps
            };
            // Walking must terminate earlier
            current_task.x = x;
            current_task.y = y;
            Some(moment)
        }
        TaskType::WelcomeAbility => {
            let cast_time = current_task.start_time + AbilityType::Welcome.busy_duration();
            Some(cast_time)
        }
    }
}

/// For the given worker, executes tasks on the DB that are due
fn execute_worker_tasks(db: &DB, worker_id: WorkerKey, village: VillageKey) -> Option<Task> {
    let mut tasks = db.past_worker_tasks(worker_id);
    let current_task = tasks.pop();
    let mut town = TownView::load_village(db, village);
    for task in tasks {
        if let Err(e) = finish_task(db, task.id, Some(task), Some(&mut town)) {
            println!("Executing task failed: {}", e)
        }
    }
    current_task
}

pub(crate) fn finish_task(
    db: &DB,
    task_id: i64,
    task: Option<Task>,
    town: Option<&mut TownView>,
) -> Result<Option<(Event, DateTime<Utc>)>, Box<dyn std::error::Error>> {
    let task = task.or_else(|| db.task(task_id));
    if let Some(task) = task {
        let mut worker = db
            .worker_priv(task.worker())
            .ok_or("Task references non-existing worker")?;
        if let Some(town) = town {
            crate::worker_actions::simulate_finish_task(&task, town, &mut worker)?;
            apply_task_to_db(db, &task, &mut worker)?;
        } else {
            let mut town = TownView::load_village(db, VillageKey(worker.home));
            crate::worker_actions::simulate_finish_task(&task, &mut town, &mut worker)?;
            apply_task_to_db(db, &task, &mut worker)?;
        }

        db.update_worker(&worker);
        db.update_worker_flag_timestamp_now(worker.key(), WorkerFlagType::Work);
        db.delete_task(&task);

        Ok(Event::load_next_worker_task(db, task.worker()))
    } else {
        // Already executed.
        Ok(None)
    }
}

fn apply_task_to_db(db: &DB, task: &Task, worker: &mut Worker) -> Result<(), String> {
    match task.task_type {
        TaskType::WelcomeAbility => {
            let a = AbilityType::Welcome;
            let (attribute, strength) = a.apply();
            let ne = NewEffect {
                hobo_id: task.target().ok_or("Ability must have a target")?.num(),
                attribute,
                strength: Some(strength),
                start_time: None, // default = now
            };
            db.insert_effect(&ne);
            db.update_ability_used_timestamp(WorkerKey(worker.id), a);
            *worker.mana.as_mut().unwrap() -= AbilityType::Welcome.mana_cost();
        }
        TaskType::CollectReward => {
            if let Some(building) = db.find_building_by_coordinates(task.x, task.y, worker.home()) {
                match building.building_type.reward_exp() {
                    Some(exp) => {
                        worker.add_exp(exp);
                        db.delete_building(&building);
                    }
                    None => {
                        return Err(format!(
                            "Tried to collect {} as reward",
                            building.building_type
                        ));
                    }
                }
            } else {
                return Err(format!("No reward to collect at {},{}", task.x, task.y));
            }
        }
        _ => { /* NOP */ }
    }
    Ok(())
}

/// (Try to) apply changes to village state that happen when a worker stops doing a given task.
/// E.g. remove unit from building.
/// Returns the time it takes until the task is actually finished.
fn simulate_finish_task<T: WorkerAction>(
    task: &T,
    town: &mut TownView,
    worker: &mut Worker,
) -> Result<Duration, String> {
    match task.task_type() {
        TaskType::Idle => Ok(Duration::milliseconds(0)),
        TaskType::Walk => Ok(worker_walk(
            town,
            worker,
            (task.x() as usize, task.y() as usize),
        )?),
        TaskType::GatherSticks | TaskType::ChopTree => {
            town.state
                .register_task_end(*task.task_type())
                .map_err(|e| e.to_string())?;
            worker_out_of_building(town, worker, (task.x() as usize, task.y() as usize))
        }
        TaskType::WelcomeAbility => {
            let a = AbilityType::Welcome;
            let duration = a.busy_duration();
            Ok(duration)
        }
        TaskType::CollectReward => {
            // Lookup object to be collected, then delete it in TownView
            // Note: DB update is separate
            let index = (task.x() as usize, task.y() as usize);
            town.state.remove(&index);
            Ok(Duration::milliseconds(0))
        }
        TaskType::Defend => Err("Task not implemented".to_owned()),
    }
}
/// (Try to) apply changes to village state that happen when a worker starts a given task.
/// E.g. add unit to a building, or pay required price (only if it is TownView), ...
fn simulate_begin_task<T: WorkerAction>(
    task: &T,
    town: &mut TownView,
    worker: &mut Worker,
) -> Result<(), String> {
    match task.task_type() {
        TaskType::Idle | TaskType::Walk | TaskType::CollectReward => Ok(()),
        TaskType::GatherSticks | TaskType::ChopTree => {
            town.state
                .register_task_begin(*task.task_type())
                .map_err(|e| e.to_string())?;
            worker_into_building(town, worker, (task.x() as usize, task.y() as usize))
        }
        TaskType::WelcomeAbility => {
            if let Some(mana) = &mut worker.mana {
                let cost = AbilityType::Welcome.mana_cost();
                if *mana >= cost {
                    *mana = *mana - cost;
                    Ok(())
                } else {
                    Err("Not enough mana".to_owned())
                }
            } else {
                Err("Worker has no mana but tries to use welcome ability".to_owned())
            }
        }
        TaskType::Defend => Err("Task not implemented".to_owned()),
    }
}

impl WorkerAction for NewTask {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn task_type(&self) -> &TaskType {
        &self.task_type
    }
    fn target(&self) -> Option<HoboKey> {
        self.target_hobo_id.map(HoboKey)
    }
}
impl WorkerAction for Task {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn task_type(&self) -> &TaskType {
        &self.task_type
    }
    fn target(&self) -> Option<HoboKey> {
        self.target_hobo_id.map(HoboKey)
    }
}
