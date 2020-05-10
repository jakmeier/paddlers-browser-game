//! Common key types for cross-layer entities.
//! The only purpose of these keys is to provide static type checks.

use crate::prelude::PadlId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VillageKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct HoboKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WorkerKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AttackKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VisitReportKey(pub PadlId);
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskKey(pub PadlId);

impl Into<i64> for VillageKey {
    fn into(self) -> i64 {
        self.0
    }
}
/// Sometimes this is preferred over into() because it has an explicit type
/// and can make the syntax cleaner
impl VillageKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Into<i64> for PlayerKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl PlayerKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Into<i64> for HoboKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl HoboKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}
impl Into<i64> for WorkerKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl WorkerKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}
impl Into<i64> for AttackKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl AttackKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}
impl Into<i64> for VisitReportKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl VisitReportKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}

impl Into<i64> for TaskKey {
    fn into(self) -> i64 {
        self.0
    }
}
impl TaskKey {
    pub fn num(&self) -> i64 {
        self.0
    }
}
