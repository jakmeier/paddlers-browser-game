//! This module deals with visitor groups (attacks) leaving towns again and calculating the outcome of the visit.
//!
//! # Attack Lifecycle
//! 1. Created                  -> TRAVELLING
//! 2. Arrival time is reached  -> ARRIVED
//! 3. Fight starts             -> ENTERED_TOWN
//! 4. Fight ends               -> REMOVED
//!
//! # How Attacks Work
//! An attacker group contains several hobos and is created at a certain time. (`departure`)
//! The arrival timestamp is computed when departing. The travel time depends on distance.
//! Hobos associated with an attack can have different speeds. For the travel time, this does not matter. 
//!
//! After arriving at the destination time, they are not entering it immediately.
//! Player input is required to let them in. Until then they are queued up in the watergate queue.
//! If the watergate queue is full and more attackers arrive, they will pop the first in the queue to make space for themselves.
//!
//! Inside he town, there is a second queue, sometimes called the resting queue. This one is for (non-hurried) hobos.
//! Units in that queue will swim to the center of the town and stay there until satisfied or pushed by another unit that takes its place.
//! Hurried visitors just go through, without interaction with the resting queue.
//!
//! A fight report is generated as soon as all visitors have left or have been satisfied.
//! Usually, the satisfaction of each visitor is only computed when time is up for an attack to be finished.
//! But there are two exceptions.
//!     1) When a player has an open browser window, the frontend can detect that a visitor is satisfied and then notify the server
//!     2) Units that wait in the town need to be checked regularly
//!
//! Effects that must be taken into consideration:
//!     * Defensive towers (flowers etc) which are only available by computing proximity
//!     * Direct effects on units, from abilities, which are explicitly stored on the db
//!
//! When a unit is defeated or leaves otherwise, it still has to stick around in the database until all units of the group are done.
//! This can be marked in the db using the status on each HoboToAttack.

use crate::db::DB;
use crate::town_view::TownView;
use chrono::NaiveDateTime;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;

pub(crate) struct AttackingHobo<'a> {
    hobo: &'a Hobo,
    attack_to_hobo: &'a AttackToHobo,
    effects: &'a [Effect],
    attack: &'a Attack,
}

impl DB {
    /// Checks if all visitors have already left (or been satisfied).
    /// If so, the visit is evaluated and a report with rewards is generated.
    pub fn maybe_evaluate_attack(&self, atk: &Attack, now: NaiveDateTime) {
        let now: Timestamp = now.into();
        let village = atk.destination();
        let active_units = self.attack_hobos_active_with_attack_info(atk);
        let town = TownView::load_village(&self, village);

        for (hobo, info) in &active_units {
            let effects = self.effects_on_hobo(hobo.key());
            let unit = AttackingHobo {
                hobo: hobo,
                attack_to_hobo: info,
                effects: &effects,
                attack: atk,
            };
            if town.hp_left(&unit, now) == 0 {
                self.set_satisfied(hobo.key(), atk.key(), true);
                if !hobo.hurried && info.released.is_none() {
                    self.release_resting_visitor(hobo.key(), atk.key());
                }
            } else if town.hobo_left_town(&unit, now) {
                self.set_satisfied(hobo.key(), atk.key(), false);
            }
        }

        // Check if all are satisfied or have left otherwise, then finish visit
        if self.attack_done(atk) {
            self.generate_report(atk);
            if atk.origin_village_id.is_none() {
                self.delete_attack_hobos(atk.key());
            }
            self.delete_attack(atk);
        }
    }

    fn generate_report(&self, atk: &Attack) {
        let mut report = NewVisitReport {
            village_id: atk.destination_village_id,
            karma: 0,
        };

        let happy_hobos = self.attack_hobos_satisfied(atk);
        report.karma = happy_hobos.len() as i64;

        use std::ops::Add;
        let feathers = happy_hobos.iter().map(reward_feathers).fold(0, i64::add);
        let sticks = happy_hobos.iter().map(reward_sticks).fold(0, i64::add);
        let logs = happy_hobos.iter().map(reward_logs).fold(0, i64::add);

        if report.karma + feathers + sticks + logs == 0 {
            return;
        }

        let vr = self.insert_visit_report(report);

        let mut rewards = vec![];
        if feathers > 0 {
            rewards.push(NewReward {
                visit_report_id: vr.id,
                resource_type: ResourceType::Feathers,
                amount: feathers,
            });
        }
        if sticks > 0 {
            rewards.push(NewReward {
                visit_report_id: vr.id,
                resource_type: ResourceType::Sticks,
                amount: sticks,
            });
        }
        if logs > 0 {
            rewards.push(NewReward {
                visit_report_id: vr.id,
                resource_type: ResourceType::Logs,
                amount: logs,
            });
        }
        self.insert_visit_report_rewards(rewards);
    }
}

/// TODO [0.1.5]
fn reward_feathers(unit: &Hobo) -> i64 {
    let f = if unit.hurried {
        (1.0 + unit.hp as f32 * unit.speed / 4.0).log2().floor()
    } else {
        (1.0 + unit.hp as f32 / 16.0).log2().ceil()
    };
    // println!("Unit {:?} gives {} feathers", unit, f);
    f as i64
}

/// TODO [0.1.5]
fn reward_sticks(unit: &Hobo) -> i64 {
    if unit.id % 20 == 0 {
        5
    } else {
        0
    }
}

/// TODO [0.1.5]
fn reward_logs(unit: &Hobo) -> i64 {
    if unit.id % 60 == 11 {
        5
    } else {
        0
    }
}

impl<'a> IAttackingHobo for AttackingHobo<'a> {
    fn max_hp(&self) -> u32 {
        self.hobo.hp as u32
    }
    fn speed(&self) -> f32 {
        self.hobo.speed
    }
    fn hurried(&self) -> bool {
        self.hobo.hurried
    }
    fn arrival(&self) -> Timestamp {
        self.attack.arrival.into()
    }
    fn released(&self) -> Option<Timestamp> {
        self.attack_to_hobo.released.map(|t| t.into())
    }
    fn effects_strength(&self) -> i32 {
        self.effects
            .iter()
            .filter(|e| e.attribute == HoboAttributeType::Health)
            .filter(|e| e.strength.is_some())
            .fold(0, |acc, e| acc + e.strength.unwrap() as i64) as i32
    }
}

impl ITownLayoutMarker for TownView {
    const LAYOUT: TownLayout = TownLayout::Basic;
}
impl IDefendingTown for TownView {
    type AuraId = i64;
    fn auras_in_range(&self, index: &Self::Index, time: Timestamp) -> Vec<(Self::AuraId, i32)> {
        let mut auras = vec![];
        for b in &self.buildings_with_aura {
            if time < b.creation.into() {
                continue;
            }
            if b.attacks_per_cycle.is_none() {
                if let (Some(range), Some(ap)) = (b.building_range, b.attack_power) {
                    let dx = (b.x - index.0 as i32).abs();
                    let dy = (b.y - index.1 as i32).abs();
                    let in_range = (dx * dx + dy * dy) as f32 <= range * range;
                    if in_range {
                        auras.push((b.id, ap));
                    }
                }
            }
        }
        auras
    }
}
