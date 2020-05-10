//! In this shared module, the full hp-computation for any given attacker is programmed.
//!
//! The traits in here define what information is required to perform the computations.
//! Based solely on this information, the computation is defined inside the traits.
//! The frontend and the backend can therefore use his computation by implementing the traits.
use super::town_layout::ITownLayout;
use super::{TOWN_RESTING_X, TOWN_X};
use crate::shared_types::*;

/// Provides information about a hobo currently attacking
pub trait IAttackingHobo {
    // TO IMPLEMENT
    fn max_hp(&self) -> u32;
    fn speed(&self) -> f32;
    fn hurried(&self) -> bool;
    fn arrival(&self) -> Timestamp;
    fn released(&self) -> Option<Timestamp>;
    fn effects_strength(&self) -> i32;

    // PROVIDED
    /// Returns the duration it takes the hobo to reach the resting place, after having reached the town.
    fn time_until_resting(&self) -> Timestamp {
        Self::s_time_until_resting(self.speed())
    }
    fn s_time_until_resting(speed: f32) -> Timestamp {
        let distance_until_resting = TOWN_X - TOWN_RESTING_X;
        Timestamp::from_float_seconds(distance_until_resting as f32 / speed)
    }
}

/// Trait for town information required to perform hp computations
pub trait IDefendingTown: ITownLayout {
    // TO IMPLEMENT
    type AuraId: Ord + PartialEq;
    fn auras_in_range(&self, index: &Self::Index, time: Timestamp) -> Vec<(Self::AuraId, i32)>;

    // PROVIDED
    fn hp_left<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: Timestamp) -> u32 {
        attacker
            .max_hp()
            .saturating_sub(self.total_damage(attacker, now) as u32)
    }
    fn total_damage<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: Timestamp) -> i32 {
        self.aura_damage(attacker, now) + attacker.effects_strength()
    }

    fn hobo_left_town<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: Timestamp) -> bool {
        if attacker.hurried() {
            let time_since_arrival = now - attacker.arrival();
            // +1 for swimming out of sight
            let distance = self.path_straight_through().len() + 1;
            time_since_arrival.seconds_float() >= distance as f32 / attacker.speed()
        } else {
            if let Some(released) = self.left_rest_place(attacker) {
                let time_since_release = now - released;
                // +1 for swimming out of sight
                let distance = self.path_from_rest_place().len() + 1;
                time_since_release.seconds_float() >= distance as f32 / attacker.speed()
            } else {
                false
            }
        }
    }
    fn aura_damage<HOBO: IAttackingHobo>(&self, attacker: &HOBO, now: Timestamp) -> i32 {
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
        now: Timestamp,
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
                println!("Released at {:?}", released);
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
        start: Timestamp,
        max_t: Timestamp,
        attacker: &HOBO,
        tiles: &[Self::Index],
    ) -> Vec<(Self::AuraId, i32)> {
        let mut out = vec![];
        let mut t = start;
        let t_per_tile = Timestamp::from_float_seconds(1.0 / attacker.speed());
        for tile in tiles {
            if t > max_t {
                break;
            }
            out.append(&mut self.auras_in_range(tile, t));
            t = t + t_per_tile;
        }
        out.sort();
        out.dedup();
        out
    }
    /// The timestamp when the resting place was left by a non-hurried hobo. May differ from hobo.released
    fn left_rest_place<HOBO: IAttackingHobo>(&self, attacker: &HOBO) -> Option<Timestamp> {
        attacker.released().map(|released| {
            let f = self.path_to_rest_place().len() as f32 / attacker.speed();
            let t = Timestamp::from_float_seconds(f);
            let started_resting = attacker.arrival() + t;
            if released > started_resting {
                released
            } else {
                started_resting
            }
        })
    }
}
