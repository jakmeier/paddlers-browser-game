use crate::models::*;
use chrono::Duration;

impl AbilityType {
    /// Returns affected attribute and strength of ability
    pub fn apply(&self) -> (HoboAttributeType, i32) {
        match self {
            AbilityType::Work => panic!("Cannot apply work to hobo"),
            AbilityType::Welcome => (HoboAttributeType::Health, 1),
        }
    }

    pub fn from_task(task: &TaskType) -> Option<AbilityType> {
        match task {
            TaskType::WelcomeAbility => Some(AbilityType::Welcome),
            TaskType::ChopTree | TaskType::GatherSticks => Some(AbilityType::Work),
            _ => None,
        }
    }

    /// How long it takes a worker to perform the ability
    pub fn busy_duration(&self) -> Duration {
        let ms = match self {
            AbilityType::Welcome => 1000,
            AbilityType::Work => 0,
        };
        Duration::milliseconds(ms)
    }

    /// How long until the ability can be used again
    pub fn cooldown(&self) -> Duration {
        let ms = match self {
            AbilityType::Welcome => 10000,
            AbilityType::Work => 0,
        };
        Duration::milliseconds(ms)
    }

    pub fn mana_cost(&self) -> i32 {
        match self {
            AbilityType::Welcome => 5,
            AbilityType::Work => 0,
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            AbilityType::Welcome => 2.0,
            AbilityType::Work => 0.0,
        }
    }
}
