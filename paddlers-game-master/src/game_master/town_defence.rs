//! This module deals with visitor groups (attacks) leaving towns again and calculating the outcome of the visit.
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
use chrono::NaiveDateTime;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;

impl DB {
    /// Checks if all visitors have already left (or been satisfied).
    /// If so, the visit is evaluated and a report with rewards is generated.
    pub fn maybe_evaluate_attack(&self, atk: &Attack, now: NaiveDateTime) {
        let time_into_fight = now - atk.arrival;
        let off = self.attack_hobos_active_with_released_flag(atk);
        let village = VillageKey(atk.destination_village_id);
        let def = self.buildings(village);
        let map_len = TOWN_X as f32;
        let halve_map_len = ((TOWN_X + 1) / 2) as f32;
        for (hobo, released) in &off {
            let mut swum_distance = hobo.speed * time_into_fight.num_milliseconds() as f32 / 1000.0;
            if hobo.hurried {
                // hurried hobos swim right through
                if swum_distance < map_len {
                    // Need to wait
                    continue;
                }
            } else {
                swum_distance = if let Some(released) = released {
                    let time_released = now - *released;
                    halve_map_len + hobo.speed * time_released.num_milliseconds() as f32 / 1000.0
                } else {
                    halve_map_len
                }
                .min(swum_distance);
            }
            let ap = aura_def_pts(&def, swum_distance as usize) as i64;
            let mut dmg = 0;
            dmg += self.damage_from_effects(hobo);
            dmg += ap;
            let happy = dmg >= hobo.hp;
            self.set_satisfied(hobo.key(), atk.key(), happy);
        }

        // Check if all are satisfied or have left otherwise, then finish visit
        if self.attack_done(atk) {
            self.generate_report(atk);
            // TODO: Move survivors back or delete them
            // defeated_units.for_each(|u| self.delete_hobo(u));
            self.delete_attack(atk);
        }
    }

    fn damage_from_effects(&self, hobo: &Hobo) -> i64 {
        self.effects_on_hobo(hobo.key())
            .iter()
            .filter(|e| e.attribute == HoboAttributeType::Health)
            .filter(|e| e.strength.is_some())
            .fold(0, |acc, e| acc + e.strength.unwrap() as i64)
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

fn aura_def_pts(def: &[Building], x_distance_swum: usize) -> u32 {
    let mut sum = 0;
    let min_x = 0.max(TOWN_X as i32 - x_distance_swum as i32) as usize;
    for d in def {
        if d.attacks_per_cycle.is_none() {
            if let (Some(_range), Some(ap)) = (d.building_range, d.attack_power) {
                if tiles_in_range(d, min_x) > 0 {
                    sum += ap as u32;
                }
            }
        }
    }
    sum
}

// This could later be extended with more map variations.
// Then, it probably makes sense to move these functions to a trait

fn tiles_in_range(b: &Building, min_x: usize) -> usize {
    if b.building_range.is_none() {
        return 0;
    }
    let mut n = 0;
    let y = TOWN_LANE_Y;
    for x in min_x..TOWN_X {
        let dx = diff(b.x, x);
        let dy = diff(b.y, y);
        let range = b.building_range.expect("No range");
        let in_range = dx * dx + dy * dy <= range * range;
        if in_range {
            n += 1;
        }
    }
    n
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

// Simple helpers

fn diff(b: i32, a: usize) -> f32 {
    (a.max(b as usize) - a.min(b as usize)) as f32
}
