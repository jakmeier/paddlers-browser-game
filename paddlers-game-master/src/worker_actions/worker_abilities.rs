use crate::db::DB;
use crate::town_view::*;
use chrono::Duration;
use paddlers_shared_lib::game_mechanics::{town::*, worker::*};
use paddlers_shared_lib::prelude::*;

pub fn worker_walk(
    town: &TownView,
    worker: &mut Worker,
    to: TileIndex,
) -> Result<Duration, String> {
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

pub fn worker_out_of_building(
    town: &mut TownView,
    _worker: &mut Worker,
    to: TileIndex,
) -> Result<Duration, String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?;
    tile_state.try_remove_entity().map_err(|e| e.to_string())?;
    Ok(Duration::milliseconds(0))
}
pub fn worker_into_building(
    town: &mut TownView,
    _worker: &mut Worker,
    to: TileIndex,
) -> Result<(), String> {
    let tile_state = town.state.get_mut(&to).ok_or("No building found")?;
    tile_state.try_add_entity().map_err(|e| e.to_string())?;
    Ok(())
}
pub(super) fn validate_ability(
    db: &DB,
    task_type: TaskType,
    worker_id: WorkerKey,
    now: chrono::NaiveDateTime,
) -> Result<(), String> {
    if let Some(ability_type) = AbilityType::from_task(&task_type) {
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
