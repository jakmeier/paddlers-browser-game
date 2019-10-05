use crate::models::{TaskType, ResourceType};

pub const fn unit_speed_to_worker_tiles_per_second(base_speed: f32) -> f32 {
    base_speed
}

pub const fn hero_max_mana() -> i32 {
    100
}

pub const fn hero_mana_regeneration_per_hour() -> i32 {
    10
}

pub fn hero_resource_collection_per_hour(task: TaskType) -> Option<(ResourceType, i32)> {
    match task {
        TaskType::ChopTree => {
            Some((ResourceType::Logs, 5))
        },
        TaskType::GatherSticks => {
            Some((ResourceType::Sticks, 60))
        },
        _ => {
            None
        }
    }
}

pub const fn hero_level_exp(now: i32) -> i32 {
    now * 100
}