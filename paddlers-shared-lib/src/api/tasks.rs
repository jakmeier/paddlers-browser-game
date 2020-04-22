use crate::api::keys::WorkerKey;
#[cfg(feature = "game_mechanics")]
use crate::game_mechanics::town::TileIndex;
use crate::models::*;
use crate::PadlId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub worker_id: WorkerKey,
    pub tasks: Vec<RawTask>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawTask {
    pub task_type: TaskType,
    pub x: usize,
    pub y: usize,
    pub target: Option<PadlId>,
}

impl RawTask {
    #[cfg(feature = "game_mechanics")]
    pub fn new(t: TaskType, i: TileIndex) -> Self {
        RawTask {
            task_type: t,
            x: i.0,
            y: i.1,
            target: None,
        }
    }
    #[cfg(feature = "game_mechanics")]
    pub fn new_with_target(t: (TaskType, Option<PadlId>), i: TileIndex) -> Self {
        RawTask {
            task_type: t.0,
            x: i.0,
            y: i.1,
            target: t.1,
        }
    }
}
