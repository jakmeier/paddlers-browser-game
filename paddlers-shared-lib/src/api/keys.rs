//! Common key types for cross-layer entities.
//! The only purpose of these keys is to provide static type checks.

use crate::prelude::*;
use serde::{Deserialize, Serialize};

// One example with `VillageKey` without macro, for readability

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VillageKey(pub PadlId);
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
#[cfg(feature = "sql_db")]
impl SqlKey<VillageKey> for Village {
    fn key(&self) -> VillageKey {
        VillageKey { 0: self.id }
    }
}

// Repetition with macros
macro_rules! object_key {
    ($object:ty, $key:ident) => {
        #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $key(pub PadlId);
        impl Into<i64> for $key {
            fn into(self) -> i64 {
                self.0
            }
        }
        impl $key {
            pub fn num(&self) -> i64 {
                self.0
            }
        }
        #[cfg(feature = "sql_db")]
        impl SqlKey<$key> for $object {
            fn key(&self) -> $key {
                $key { 0: self.id }
            }
        }
    };
}

object_key!(Attack, AttackKey);
object_key!(Hobo, HoboKey);
object_key!(Player, PlayerKey);
object_key!(Stream, StreamKey);
object_key!(Task, TaskKey);
object_key!(VisitReport, VisitReportKey);
object_key!(Worker, WorkerKey);
