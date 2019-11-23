use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::game_mechanics::town::*;
use crate::db::DB;
use chrono::Duration;

impl DB {
    pub fn maybe_attack_now(&self, atk: &Attack, time_into_fight: Duration) {

        let off = self.attack_hobos(atk);

        let seconds = seconds_to_complete(&off).ceil();
        if time_into_fight > Duration::milliseconds((seconds * 1_000.0) as i64){
            let village = VillageKey(atk.destination_village_id);
            let def = self.buildings(village);
            self.execute_fight(&def, &off, village);
            self.delete_attack(atk);
        }

    }

    fn execute_fight(&self, defenders: &[Building], attackers: &[Hobo], village: VillageKey) {

        // println!("Fight!");
        // println!("{:#?} against {:#?}", defenders, attackers);
        let ap = aura_def_pts(defenders) as i64;
        // println!("Aura def = {}", ap);

        let defeated_units = attackers.into_iter()
            .map(|a| (a, a.hp - ap as i64))
            .map(|(a, hp)| (a, hp - self.damage_from_effects(a)) )
            .filter(|(_, hp)| *hp <= 0 )
            .map(|(a, _)| a );
        let player = self.village(village).expect("Village of fight").owner();
        self.collect_reward(defeated_units.clone(), village, player);
        defeated_units.for_each(|u| self.delete_hobo(u));

        // TODO: Move survivors back
    }

    fn damage_from_effects(&self, hobo: &Hobo) -> i64 {
        self.effects_on_hobo(hobo.key())
            .iter()
            .filter(|e| e.attribute == HoboAttributeType::Health )
            .filter(|e| e.strength.is_some() )
            .fold(0, |acc, e| acc + e.strength.unwrap() as i64)
    }
}

fn aura_def_pts(def: &[Building]) -> u32 {
    let mut sum = 0;
    for d in def {
        if d.attacks_per_cycle.is_none() {
            if let (Some(_range), Some(ap)) = (d.building_range, d.attack_power) {
                if tiles_in_range(d) > 0 {
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
    let mut n = 0;
    let y = TOWN_LANE_Y;
    for x in 0 .. TOWN_X {
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
fn seconds_to_complete(units: &[Hobo]) -> f32 {
    let slowest_speed =
        units.iter()
        .map(|u| u.speed)
        .fold(std::f32::MAX, f32::min);
    let map_len = TOWN_X as f32;
    
    map_len / slowest_speed
}

// Simple helpers

fn diff(b: i32, a: usize) -> f32 {
    ( a.max(b as usize) - a.min(b as usize) ) as f32
}

    