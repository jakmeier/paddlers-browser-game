use crate::prelude::*;

pub trait SqlKey<K> {
    fn key(&self) -> K;
}

impl Task {
    pub fn worker(&self) -> WorkerKey {
        WorkerKey(self.worker_id)
    }
}

impl Worker {
    pub fn home(&self) -> VillageKey {
        VillageKey(self.home)
    }
}

impl Village {
    pub fn owner(&self) -> Option<PlayerKey> {
        Some(PlayerKey(self.player_id?))
    }
}

impl VisitReport {
    pub fn village(&self) -> VillageKey {
        VillageKey(self.village_id)
    }
}

impl Attack {
    pub fn destination(&self) -> VillageKey {
        VillageKey(self.destination_village_id)
    }
    pub fn origin(&self) -> Option<VillageKey> {
        self.origin_village_id.map(VillageKey)
    }
}

impl Building {
    pub fn village(&self) -> VillageKey {
        VillageKey(self.village_id)
    }
}