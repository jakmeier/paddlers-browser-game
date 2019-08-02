use chrono::Duration;
use crate::models::TaskType;

pub fn tree_size(age: Duration) -> usize {
    match age.num_hours() {
        h if h < 1 => 1,
        h if h < 4 => 2,
        h if h <= 45 => 3 + h as usize / 9,
        h if h < 72 => 9,
        _ => 10,
    }
}

impl TaskType {
    pub fn required_forest_size(&self) -> usize {
        match self {
            TaskType::GatherSticks => 3,
            TaskType::ChopTree => 20,
            _ => 0,
        }
    }
}