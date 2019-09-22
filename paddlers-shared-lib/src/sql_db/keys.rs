use crate::prelude::*;

pub trait SqlKey<K> {
    fn key(&self) -> K;
}

impl SqlKey<VillageKey> for Village {
    fn key(&self) -> VillageKey {
        VillageKey(self.id)
    }
}

impl SqlKey<WorkerKey> for Worker {
    fn key(&self) -> WorkerKey {
        WorkerKey(self.id)
    }
}
impl SqlKey<HoboKey> for Hobo {
    fn key(&self) -> HoboKey {
        HoboKey(self.id)
    }
}
