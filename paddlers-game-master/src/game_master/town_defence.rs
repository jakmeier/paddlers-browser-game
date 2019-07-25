use paddlers_shared_lib::models::*;
use crate::db::DB;
use paddlers_shared_lib::sql::GameDB;
use chrono::Duration;

impl DB {
    pub fn maybe_attack_now(&self, atk: &Attack, time_into_fight: Duration) {

        let off = self.attack_units(atk);

        let cycle_secs = 10;
        let cycles = cycles_to_complete(&off).ceil();
        if time_into_fight > Duration::seconds(cycles as i64 * cycle_secs) {
            let def = self.buildings();
            self.execute_fight(&def, &off);
            self.delete_attack(atk);
        }

    }

    fn execute_fight(&self, defenders: &[Building], attackers: &[Unit]) {

        println!("Fight!");
        // println!("{:#?} against {:#?}", defenders, attackers);
        let ap = aura_def_pts(defenders);
        println!("Aura def = {}", ap);

        let defeated_units = attackers.iter().filter(|a| (a.hp as u32) <= ap );
        self.collect_reward(defeated_units.clone());
        defeated_units.for_each(|u| self.delete_unit(u));

        // TODO: Move survivors back
    }
}

fn aura_def_pts(def: &[Building]) -> u32 {
    let mut sum = 0;
    for d in def {
        if d.attacks_per_cycle.is_none() {
            if let (Some(_range), Some(ap)) = (d.building_range, d.attack_power) {
                if tiles_in_range(d) > 0 {
                    // TODO: AP should be integer
                    sum += ap as u32;
                }
            }
        }
    }
    sum
}

// This could later be extended with more map variations.
// Then, it probably makes sense to move these functions to a trait

fn tiles_in_range(b: &Building) -> usize {
    if b.building_range.is_none() {
        return 0;
    }
    // Map is a straight horizontal line with y = 6 and x \in [0,23]
    let mut n = 0;
    let y = 6;
    for x in 0 .. 23 {
        let dx = diff(b.x, x);
        let dy = diff(b.y, y);
        let range = b.building_range.expect("No range");
        let in_range = dx*dx + dy*dy <= range * range;
        if in_range {
            n += 1;
        }
    }
    n
}
fn cycles_to_complete(units: &[Unit]) -> f32 {
    let slowest_speed =
        units.iter()
        .map(|u| u.speed)
        .fold(0.0, f32::max);
    let map_len = 23.0;
    
    slowest_speed * map_len
}

// Simple helpers

fn diff(b: i32, a: usize) -> f32 {
    ( a.max(b as usize) - a.min(b as usize) ) as f32
}

    