//! Common key types for cross-layer entities.
//! The only purpose of these keys is to provide static type checks.

use serde::{Serialize, Deserialize};
use crate::PadlId;

#[derive(Clone,Copy,Debug, Serialize, Deserialize)]
pub struct VillageKey(pub PadlId);
#[derive(Clone,Copy,Debug, Serialize, Deserialize)]
pub struct HoboKey(pub PadlId);
#[derive(Clone,Copy,Debug, Serialize, Deserialize)]
pub struct WorkerKey(pub PadlId);

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