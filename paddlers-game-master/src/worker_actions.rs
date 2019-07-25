use actix::prelude::*;
use paddlers_shared_lib::api::tasks::*;
use paddlers_shared_lib::game_mechanics::{
    town::*,
    worker::*,
};
use paddlers_shared_lib::prelude::*;
use chrono::{NaiveDateTime, Duration, Utc};
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

pub (crate) fn validate_task_list(db: &DB, tl: &TaskList, village_id: i64) -> Result<Vec<NewTask>, Box<dyn std::error::Error>> {

    let unit_id = tl.unit_id;

    // 1 get current task(s)
    let current_tasks = db.unit_tasks(unit_id);

    // 2 Load relevant data into memory 
    let town = TownView::load_village(db, village_id);
    let mut unit = db.unit(unit_id).ok_or("Unit does not exist")?;

    // 3 check if user can interrupt current task
    // & calculate earliest time when new tasks can be accepted
    let mut timestamp = next_possible_interrupt(&current_tasks).ok_or("Cannot interrupt current task.")?;

    // 4 iterate tasks and match for task types
    let mut tasks = vec![];

    for task in tl.tasks.iter() {
        let new_task = NewTask {
            unit_id: unit_id,
            task_type: task.task_type,
            x: task.x as i32,
            y: task.y as i32,
            start_time: Some(timestamp),
        };
        let duration = simulate_task(&new_task, &town, &mut unit)?;
        tasks.push(new_task);
        timestamp += duration;
    }
    Ok(tasks)
}
pub (crate) fn replace_unit_tasks(db: &DB, worker: &Addr<TownWorker>, unit_id: i64, tasks: &[NewTask]) {
    db.flush_task_queue(unit_id);
    db.insert_tasks(tasks);
    let current_task = execute_unit_tasks(db, unit_id).expect("Unit has no current task");
    if let Some(next_task) = db.earliest_future_task(unit_id) {
        let event = Event::UnitTask{ task_id: current_task.id};
        worker.send(TownWorkerEventMsg(event, Utc.from_utc_datetime(&next_task.start_time))).wait().expect("Send msg to actor");
    }
}

fn next_possible_interrupt(_current_tasks: &[Task]) -> Option<NaiveDateTime> {
    // TODO: Make sure the units are aligned in the tiles
    let now = chrono::Utc::now().naive_utc();
    Some(now)
}

fn execute_unit_tasks(db: &DB, unit_id: i64) -> Option<Task> {
    let mut tasks = db.unit_tasks(unit_id);
    let current_task = tasks.pop();
    let village_id = 1; // TODO: Village id
    let town = TownView::load_village(db, village_id);
    for task in tasks {
        execute_task(db, task.id, Some(task), Some(&town)).map_err(
            |e| println!("Executing task failed: {}", e)
        ).unwrap();
    }
    current_task
}

pub (crate) fn execute_task(
    db: &DB, 
    task_id: i64, 
    task: Option<Task>, 
    town: Option<&TownView>
) -> Result<(), Box<dyn std::error::Error>> 
{
    let task = task.or_else(|| db.task(task_id)).ok_or("No task to execute found")?;
    let mut unit = db.unit(task.unit_id).ok_or("Task references non-existing unit")?;
    if let Some(town) = town {
        crate::worker_actions::simulate_task(&task, &town, &mut unit)?;
    } else {
        let town = TownView::load_village(db, unit.home);
        crate::worker_actions::simulate_task(&task, &town, &mut unit)?;
    }
    
    db.update_unit(&unit);
    db.delete_task(&task);
    Ok(())
}

fn simulate_task<T: WorkerAction> (
    task: &T, 
    town: &TownView, 
    unit: &mut Unit
) -> Result<Duration, String> 
{
    match task.task_type() {
        TaskType::Idle => Ok(Duration::milliseconds(0)),
        TaskType::Walk => Ok(worker_walk(town, unit, (task.x() as usize, task.y() as usize))?),
        _ => Err("Task not implemented".to_owned())
    }
}

fn worker_walk(town: &TownView, unit: &mut Unit, to: TileIndex) -> Result<Duration, String> {
    let from = (unit.x as usize, unit.y as usize);
    if !town.path_walkable(from, to) {
        return Err("Cannot walk this way.".to_owned());
    }
    let speed = unit_speed_to_worker_tiles_per_second(unit.speed);
    let distance = distance2(from, to).sqrt();
    let seconds = distance / speed;
    unit.x = to.0 as i32;
    unit.y = to.1 as i32;
    Ok(Duration::microseconds((seconds * 1_000_000.0) as i64))
}

// fn earliest_future_task(tasks: &[Task], now: DateTime<Utc>) -> Option<&Task> {
//     let now = now.naive_utc();
//     let mut iter = tasks.iter().filter(|t| t.start_time >= now);

//     let task = iter.next();
//     task.map( 
//         |task|
//         iter.fold(
//             task, 
//             |a, b| 
//                 if a.start_time <= b.start_time { a } 
//                 else { b }
//         )
//     )
// }

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