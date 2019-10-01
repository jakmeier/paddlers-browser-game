use actix::prelude::*;
use paddlers_shared_lib::api::tasks::*;
use paddlers_shared_lib::game_mechanics::{
    town::*,
    worker::*,
};
use paddlers_shared_lib::prelude::*;
use chrono::{NaiveDateTime, DateTime, Duration, Utc};
use chrono::offset::TimeZone;
use crate::db::DB;
use crate::town_view::*;
use crate::game_master::town_worker::*;
use crate::game_master::event::*;

trait WorkerAction {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn task_type(&self) -> &TaskType;
    fn target(&self) -> Option<HoboKey>;
}
pub struct ValidatedTaskList {
    pub new_tasks: Vec<NewTask>,
    pub update_tasks: Vec<Task>,
}
pub (crate) fn validate_task_list(db: &DB, tl: &TaskList, village_id: i64) -> Result<ValidatedTaskList, Box<dyn std::error::Error>> {

    let worker_id = tl.worker_id;

    // Load relevant data into memory 
    let mut town = TownView::load_village(db, village_id);
    let mut worker = db.worker(worker_id).ok_or("Worker does not exist")?;

    // check timing and effect of current task interruption
    let mut current_task = db.current_task(worker.key()).expect("Must have a current task");
    let mut timestamp = interrupt_task(&mut current_task, &worker).ok_or("Cannot interrupt current task.")?;
    worker.x = current_task.x;
    worker.y = current_task.y;

    // iterate tasks and match for task types
    let mut tasks = vec![];

    for task in tl.tasks.iter() {
        
        // Validate target hobo exists if there is one
        if let Some(target_id) =  task.target {
            db.hobo(target_id).ok_or("No such hobo id")?;
        }

        validate_ability(db, task.task_type, worker_id, timestamp)?;

        let new_task = NewTask {
            worker_id: worker_id,
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
    Ok( ValidatedTaskList {
            new_tasks: tasks,
            update_tasks: vec![current_task],
        }
    )
}
pub (crate) fn replace_worker_tasks(db: &DB, worker: &Addr<TownWorker>, worker_id: i64, tasks: &[NewTask]) {
    db.flush_task_queue(worker_id);
    let _inserted = db.insert_tasks(tasks);
    let current_task = execute_worker_tasks(db, worker_id).expect("Worker has no current task");
    if let Some(next_task) = db.earliest_future_task(worker_id) {
        let event = Event::WorkerTask{ task_id: current_task.id};
        worker.send(TownWorkerEventMsg(event, Utc.from_utc_datetime(&next_task.start_time))).wait().expect("Send msg to actor");
    }
}

fn interrupt_task(current_task: &mut Task, worker: &Worker) -> Option<NaiveDateTime> {
    match current_task.task_type {
        TaskType::Idle 
        | TaskType::ChopTree
        | TaskType::Defend
        | TaskType::GatherSticks
        => 
        {
            let now = chrono::Utc::now().naive_utc();
            Some(now)
        },
        TaskType::Walk => {
            let speed = unit_speed_to_worker_tiles_per_second(worker.speed) as f64;
            let time_so_far: Duration = Utc::now().naive_utc() - current_task.start_time;
            let steps = (speed * time_so_far.num_microseconds().unwrap() as f64 / 1_000_000.0).ceil() as i32;
            let total_time = steps as f64 / speed;
            let moment = current_task.start_time + chrono::Duration::microseconds((total_time * 1_000_000.0) as i64);
            let dx = current_task.x - worker.x;
            let dy = current_task.y - worker.y;
            let x = if dx == 0 { worker.x } else if dx < 0 { worker.x - steps } else { worker.x + steps};
            let y = if dy == 0 { worker.y } else if dy < 0 { worker.y - steps } else { worker.y + steps};
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
fn execute_worker_tasks(db: &DB, worker_id: i64) -> Option<Task> {
    let mut tasks = db.past_worker_tasks(worker_id);
    let current_task = tasks.pop();
    let village_id = 1; // TODO: Village id
    let mut town = TownView::load_village(db, village_id);
    for task in tasks {
        finish_task(db, task.id, Some(task), Some(&mut town)).map_err(
            |e| println!("Executing task failed: {}", e)
        ).unwrap();
    }
    current_task
}

pub (crate) fn finish_task(
    db: &DB, 
    task_id: i64, 
    task: Option<Task>, 
    town: Option<&mut TownView>
) -> Result<Option<(Event, DateTime<Utc>)>, Box<dyn std::error::Error>> 
{
    let task = task.or_else(|| db.task(task_id));
    if let Some(task) = task {
        let mut worker = db.worker(task.worker_id).ok_or("Task references non-existing worker")?;
        if let Some(town) = town {
            crate::worker_actions::simulate_finish_task(&task, town, &mut worker)?;
            apply_task_to_db(db, &task, &mut worker)?;
        } else {
            let mut town = TownView::load_village(db, worker.home);
            crate::worker_actions::simulate_finish_task(&task, &mut town, &mut worker)?;
            apply_task_to_db(db, &task, &mut worker)?;
        }
        
        db.update_worker(&worker);
        db.update_worker_flag_timestamp_now(worker.key(), WorkerFlagType::Work);
        db.delete_task(&task);

        Ok(Event::load_next_worker_task(db, task.worker_id))
    } else {
        // Already executed.
        Ok(None)
    }
}

fn apply_task_to_db(
    db: &DB,
    task: &Task,
    worker: &mut Worker,
) -> Result<(), String> {
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
        },
        _ => { /* NOP */ },
    }
    Ok(())
}

fn simulate_finish_task<T: WorkerAction> (
    task: &T, 
    town: &mut TownView, 
    worker: &mut Worker
) -> Result<Duration, String> 
{
    match task.task_type() {
        TaskType::Idle => Ok(Duration::milliseconds(0)),
        TaskType::Walk => Ok(worker_walk(town, worker, (task.x() as usize, task.y() as usize))?),
        TaskType::GatherSticks 
        | TaskType::ChopTree 
            => {
                town.state.register_task_end(*task.task_type())
                    .map_err(|e| e.to_string())?;
                worker_out_of_building(town, worker, (task.x() as usize, task.y() as usize))
            },
        TaskType::WelcomeAbility => {
            let a = AbilityType::Welcome;
            let duration = a.busy_duration();
            Ok(duration)
        },
        _ => Err("Task not implemented".to_owned())
    }
}
fn simulate_begin_task<T: WorkerAction> (
    task: &T, 
    town: &mut TownView, 
    worker: &mut Worker
) -> Result<(), String> 
{
    match task.task_type() {
        TaskType::Idle | TaskType::Walk 
            => Ok(()),
        TaskType::GatherSticks 
        | TaskType::ChopTree 
            => {
                town.state.register_task_begin(*task.task_type()).map_err(|e| e.to_string())?;
                worker_into_building(town, worker, (task.x() as usize, task.y() as usize))
            },
        TaskType::WelcomeAbility => {
            if let Some(mana) = &mut worker.mana {
                let cost  = AbilityType::Welcome.mana_cost();
                if *mana >= cost {
                    *mana = *mana - cost;
                    Ok(())
                }
                else {
                    Err("Not enough mana".to_owned())
                }
            } else {
                Err("Worker has no mana but tries to use welcome ability".to_owned())
            }
        },
        _ => Err("Task not implemented".to_owned())
    }
}

fn worker_walk(town: &TownView, worker: &mut Worker, to: TileIndex) -> Result<Duration, String> {
    let from = (worker.x as usize, worker.y as usize);
    if !town.path_walkable(from, to) {
        return Err(format!("Cannot walk this way. {:?} -> {:?}", from, to));
    }
    let speed = unit_speed_to_worker_tiles_per_second(worker.speed);
    let distance = distance2(from, to).sqrt();
    let seconds = distance / speed;
    worker.x = to.0 as i32;
    worker.y = to.1 as i32;
    Ok(Duration::microseconds((seconds * 1_000_000.0) as i64))
}

fn worker_out_of_building(town: &mut TownView, _worker: &mut Worker, to: TileIndex) -> Result<Duration, String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?; 
    tile_state.try_remove_entity().map_err(|e| e.to_string())?;
    Ok(Duration::milliseconds(0))
}
fn worker_into_building(town: &mut TownView, _worker: &mut Worker, to: TileIndex) -> Result<(), String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?; 
    tile_state.try_add_entity().map_err(|e| e.to_string())?;
    Ok(())
}
fn validate_ability(db: &DB, task_type: TaskType, worker_id: i64, now: chrono::NaiveDateTime) -> Result<(), String> {
    if let Some(ability_type) = AbilityType::from_task(&task_type)
    {
        // TODO: Range checks
        // let range = ability_type.range();
        // TODO: Take movement of visitor into account

        if let Some(a) = db.worker_ability(worker_id, ability_type) {
            if let Some(last_used) = a.last_used {
                let free_to_use = last_used + ability_type.cooldown();
                if free_to_use > now {
                    return Err("Cooldown not ready".to_owned());
                }
            }
        } else {
            return Err("Worker does not have this ability".to_owned());
        }
    }
    Ok(())
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