use serde::{Serialize, Deserialize};
use crate::models::*;

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

