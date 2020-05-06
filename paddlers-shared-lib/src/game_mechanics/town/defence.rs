//! In this shared module, the full hp-computation for any given attacker is programmed.
//!
//! The traits in here define what information is required to perform the computations.
//! Based solely on this information, the computation is defined inside the traits.
//! The frontend and the backend can therefore use his computation by implementing the traits.
use super::town_layout::ITownLayout;

/// Provides information about a hobo currently attacking
pub trait IAttackingHobo {
    fn max_hp(&self) -> u32;
    fn speed(&self) -> f32;
    fn hurried(&self) -> bool;
    fn arrival(&self) -> i64;
    fn released(&self) -> Option<i64>;
    fn effects_strength(&self) -> i32;
}

/// Trait for town information required to perform hp computations
pub trait IDefendingTown: ITownLayout {
    type AuraId: Ord + PartialEq;
    fn auras_in_range(&self, index: &Self::Index, time: i64) -> Vec<(Self::AuraId, i32)>;

    fn hp_left<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: i64) -> u32 {
        attacker
            .max_hp()
            .saturating_sub(self.total_damage(attacker, now) as u32)
    }
    fn total_damage<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: i64) -> i32 {
        self.aura_damage(attacker, now) + attacker.effects_strength()
    }

    fn hobo_left_town<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: i64) -> bool {
        if attacker.hurried() {
            let time_since_arrival = (now - attacker.arrival()) as f32 / 1000.0;
            // +1 for swimming out of sight
            let distance = self.path_straight_through().len() + 1;
            time_since_arrival >= distance as f32 / attacker.speed()
        } else {
            if let Some(released) = self.left_rest_place(attacker) {
                let time_since_release = (now - released) as f32 / 1000.0;
                // +1 for swimming out of sight
                let distance = self.path_from_rest_place().len() + 1;
                time_since_release >= distance as f32 / attacker.speed()
            } else {
                false
            }
        }
    }
    fn aura_damage<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: i64) -> i32 {
        let auras = self.touched_auras(attacker, now);
        let dmg = Self::damage(&auras);
        dmg
    }
    fn damage(auras: &[(Self::AuraId, i32)]) -> i32 {
        auras.iter().fold(0, |acc, aura| acc + aura.1)
    }
    fn touched_auras<HOBO: IAttackingHobo>(
        &self,
        attacker: &HOBO,
        now: i64,
    ) -> Vec<(Self::AuraId, i32)> {
        let mut auras = vec![];

        if attacker.hurried() {
            let tiles = self.path_straight_through();
            auras.append(&mut self.touched_auras_on_path(
                attacker.arrival(),
                now,
                attacker,
                &tiles,
            ));
        } else {
            let tiles = self.path_to_rest_place();
            auras.append(&mut self.touched_auras_on_path(
                attacker.arrival(),
                now,
                attacker,
                &tiles,
            ));
            if let Some(released) = self.left_rest_place(attacker) {
                let tiles = self.path_from_rest_place();
                auras.append(&mut self.touched_auras_on_path(released, now, attacker, &tiles));
            }
        }
        auras.sort();
        auras.dedup();
        auras
    }
    fn touched_auras_on_path<HOBO: IAttackingHobo>(
        &self,
        start: i64,
        max_t: i64,
        attacker: &HOBO,
        tiles: &[Self::Index],
    ) -> Vec<(Self::AuraId, i32)> {
        let mut out = vec![];
        let mut t = start;
        let t_per_tile = (1000.0 / attacker.speed()) as i64;
        for tile in tiles {
            t += t_per_tile;
            if t > max_t {
                break;
            }
            out.append(&mut self.auras_in_range(tile, t));
        }
        out.sort();
        out.dedup();
        out
    }
    /// The timestamp when the resting place was left by a non-hurried hobo. May differ from hobo.released
    fn left_rest_place<HOBO: IAttackingHobo>(&self, attacker: &HOBO) -> Option<i64> {
        attacker.released().map(|released| {
            let t = self.path_to_rest_place().len() as f32 / attacker.speed() * 1000.0;
            let started_resting = attacker.arrival() + t as i64;
            released.max(started_resting)
        })
    }
}
