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
}
pub struct ValidatedTaskList {
    pub new_tasks: Vec<NewTask>,
    pub update_tasks: Vec<Task>,
}
pub (crate) fn validate_task_list(db: &DB, tl: &TaskList, village_id: i64) -> Result<ValidatedTaskList, Box<dyn std::error::Error>> {

    let unit_id = tl.unit_id;

    // Load relevant data into memory 
    let mut town = TownView::load_village(db, village_id);
    let mut unit = db.unit(unit_id).ok_or("Unit does not exist")?;

    // check timing and effect of current task interruption
    let mut current_task = db.current_task(unit.id).expect("Must have a current task");
    let mut timestamp = interrupt_task(&mut current_task, &unit).ok_or("Cannot interrupt current task.")?;
    unit.x = current_task.x;
    unit.y = current_task.y;

    // iterate tasks and match for task types
    let mut tasks = vec![];

    for task in tl.tasks.iter() {
        let new_task = NewTask {
            unit_id: unit_id,
            task_type: task.task_type,
            x: task.x as i32,
            y: task.y as i32,
            start_time: Some(timestamp),
        };
        simulate_begin_task(&new_task, &mut town, &mut unit)?;
        let duration = simulate_finish_task(&new_task, &mut town, &mut unit)?;
        tasks.push(new_task);
        timestamp += duration;
    }
    Ok( ValidatedTaskList {
            new_tasks: tasks,
            update_tasks: vec![current_task],
        }
    )
}
pub (crate) fn replace_unit_tasks(db: &DB, worker: &Addr<TownWorker>, unit_id: i64, tasks: &[NewTask]) {
    db.flush_task_queue(unit_id);
    let _inserted = db.insert_tasks(tasks);
    let current_task = execute_unit_tasks(db, unit_id).expect("Unit has no current task");
    if let Some(next_task) = db.earliest_future_task(unit_id) {
        let event = Event::UnitTask{ task_id: current_task.id};
        worker.send(TownWorkerEventMsg(event, Utc.from_utc_datetime(&next_task.start_time))).wait().expect("Send msg to actor");
    }
}

fn interrupt_task(current_task: &mut Task, unit: &Unit) -> Option<NaiveDateTime> {
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
            let speed = unit_speed_to_worker_tiles_per_second(unit.speed) as f64;
            let time_so_far: Duration = Utc::now().naive_utc() - current_task.start_time;
            let steps = (speed * time_so_far.num_microseconds().unwrap() as f64 / 1_000_000.0).ceil() as i32;
            let total_time = steps as f64 / speed;
            let moment = current_task.start_time + chrono::Duration::microseconds((total_time * 1_000_000.0) as i64);
            let dx = current_task.x - unit.x;
            let dy = current_task.y - unit.y;
            let x = if dx == 0 { unit.x } else if dx < 0 { unit.x - steps } else { unit.x + steps};
            let y = if dy == 0 { unit.y } else if dy < 0 { unit.y - steps } else { unit.y + steps};
            // Walking must terminate earlier
            current_task.x = x;
            current_task.y = y;
            Some(moment)
        }
    }
}

/// For the given unit, executes tasks on the DB that are due
fn execute_unit_tasks(db: &DB, unit_id: i64) -> Option<Task> {
    let mut tasks = db.past_unit_tasks(unit_id);
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
        let mut unit = db.unit(task.unit_id).ok_or("Task references non-existing unit")?;
        if let Some(town) = town {
            crate::worker_actions::simulate_finish_task(&task, town, &mut unit)?;
        } else {
            let mut town = TownView::load_village(db, unit.home);
            crate::worker_actions::simulate_finish_task(&task, &mut town, &mut unit)?;
        }
        
        db.update_unit(&unit);
        db.delete_task(&task);

        Ok(Event::load_next_unit_task(db, task.unit_id))
    } else {
        // Already executed.
        Ok(None)
    }
}

fn simulate_finish_task<T: WorkerAction> (
    task: &T, 
    town: &mut TownView, 
    unit: &mut Unit
) -> Result<Duration, String> 
{
    match task.task_type() {
        TaskType::Idle => Ok(Duration::milliseconds(0)),
        TaskType::Walk => Ok(worker_walk(town, unit, (task.x() as usize, task.y() as usize))?),
        TaskType::GatherSticks 
        | TaskType::ChopTree 
            => 
            worker_out_of_building(town, unit, (task.x() as usize, task.y() as usize)),
        _ => Err("Task not implemented".to_owned())
    }
}
fn simulate_begin_task<T: WorkerAction> (
    task: &T, 
    town: &mut TownView, 
    unit: &mut Unit
) -> Result<(), String> 
{
    match task.task_type() {
        TaskType::Idle | TaskType::Walk 
            => Ok(()),
        TaskType::GatherSticks 
        | TaskType::ChopTree 
            => 
            worker_into_building(town, unit, (task.x() as usize, task.y() as usize)),
        _ => Err("Task not implemented".to_owned())
    }
}

fn worker_walk(town: &TownView, unit: &mut Unit, to: TileIndex) -> Result<Duration, String> {
    let from = (unit.x as usize, unit.y as usize);
    if !town.path_walkable(from, to) {
        return Err(format!("Cannot walk this way. {:?} -> {:?}", from, to));
    }
    let speed = unit_speed_to_worker_tiles_per_second(unit.speed);
    let distance = distance2(from, to).sqrt();
    let seconds = distance / speed;
    unit.x = to.0 as i32;
    unit.y = to.1 as i32;
    Ok(Duration::microseconds((seconds * 1_000_000.0) as i64))
}

fn worker_out_of_building(town: &mut TownView, _unit: &mut Unit, to: TileIndex) -> Result<Duration, String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?; 
    tile_state.try_remove_entity()?;
    Ok(Duration::milliseconds(0))
}
fn worker_into_building(town: &mut TownView, _unit: &mut Unit, to: TileIndex) -> Result<(), String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?; 
    tile_state.try_add_entity()?;
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
}