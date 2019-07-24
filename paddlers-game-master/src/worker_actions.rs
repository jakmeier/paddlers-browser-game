use paddlers_shared_lib::api::tasks::*;
use paddlers_shared_lib::game_mechanics::{
    town::*,
    worker::*,
};
use paddlers_shared_lib::prelude::*;
use chrono::{NaiveDateTime, Duration};
use crate::db::DB;
use crate::town_view::*;

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
pub (crate) fn run_tasks(db: &DB, tasks: &[NewTask]) {
    // TODO! Actually run tasks when they are due
    db.insert_tasks(tasks);
}

fn next_possible_interrupt(_current_tasks: &[Task]) -> Option<NaiveDateTime> {
    // TODO: Make sure the units are aligned in the tiles
    let now = chrono::Utc::now().naive_utc();
    Some(now)
}

// Note: Would be very desirable to somehow reuse this when actually executing task for change on DB
fn simulate_task(task: &NewTask, town: &TownView, unit: &mut Unit) -> Result<Duration, String> {
    match task.task_type {
        TaskType::Idle => Ok(Duration::milliseconds(0)),
        TaskType::Walk => Ok(worker_walk(town, unit, (task.x as usize, task.y as usize))?),
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