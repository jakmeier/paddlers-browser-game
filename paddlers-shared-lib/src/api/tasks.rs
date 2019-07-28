use serde::{Serialize, Deserialize};
use crate::models::*;
#[cfg(feature = "game_mechanics")] 
use crate::game_mechanics::town::TileIndex;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub unit_id: i64,
    pub tasks: Vec<RawTask>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawTask {
    pub task_type: TaskType,
    pub x: usize,
    pub y: usize,
}

impl RawTask {
    #[cfg(feature = "game_mechanics")] 
    pub fn new(t: TaskType, i: TileIndex) -> Self {
        RawTask {
            task_type: t,
            x: i.0,
            y: i.1,
        }
    }
}

