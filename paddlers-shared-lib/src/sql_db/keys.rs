use crate::prelude::*;

pub trait SqlKey<K> {
    fn key(&self) -> K;
}

impl SqlKey<PlayerKey> for Player {
    fn key(&self) -> PlayerKey {
        PlayerKey(self.id)
    }
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
impl SqlKey<VisitReportKey> for VisitReport {
    fn key(&self) -> VisitReportKey {
        VisitReportKey(self.id)
    }
}
impl SqlKey<AttackKey> for Attack {
    fn key(&self) -> AttackKey {
        AttackKey(self.id)
    }
}

impl SqlKey<TaskKey> for Task {
    fn key(&self) -> TaskKey {
        TaskKey(self.id)
    }
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
