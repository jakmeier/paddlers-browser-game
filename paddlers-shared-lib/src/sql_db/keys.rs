use crate::prelude::*;

#[derive(Clone,Copy,Debug)]
pub struct VillageKey(pub PadlId);

pub trait SqlKey<K> {
    fn key(&self) -> K;
}

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

impl SqlKey<VillageKey> for Village {
    fn key(&self) -> VillageKey {
        VillageKey(self.id)
    }
}
