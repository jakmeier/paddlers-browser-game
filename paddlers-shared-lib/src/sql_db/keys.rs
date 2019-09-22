use crate::prelude::*;

pub trait SqlKey<K> {
    fn key(&self) -> K;
}

impl SqlKey<VillageKey> for Village {
    fn key(&self) -> VillageKey {
        VillageKey(self.id)
    }
}

impl SqlKey<UnitKey> for Unit {
    fn key(&self) -> UnitKey {
        UnitKey(self.id)
    }
}
