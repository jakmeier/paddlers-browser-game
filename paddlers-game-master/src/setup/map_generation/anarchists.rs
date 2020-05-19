//! For generating AI villages with content

use crate::buildings::BuildingFactory;
use crate::db::DB;
use crate::setup::map_generation::Lcg;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;

const HOBOS_PER_TOWN: usize = 3;

impl DB {
    pub fn generate_anarchist_town_content(
        &self,
        village: VillageKey,
        lcg: &mut Lcg,
    ) -> Result<(), &'static str> {
        self.add_random_forest_to_village(village, lcg);
        self.generate_anarchist_hobos(HOBOS_PER_TOWN, village, lcg)?;
        Ok(())
    }
    fn add_random_forest_to_village(&self, village: VillageKey, lcg: &mut Lcg) {
        // Two contiguous forests in the top corners
        let mut left = lcg.next_in_range(0, 2 * TOWN_X as u64 / 3);
        let mut right = lcg.next_in_range(TOWN_X as u64 / 3, TOWN_X as u64);
        for y in 0..TOWN_LANE_Y {
            left += lcg.next_in_range(0, 4);
            left = left.saturating_sub(3);
            right -= lcg.next_in_range(0, 4);
            right += 3;
            right = right.min(TOWN_X as u64);
            right = right.max(left + 1);

            for x in 0..left as usize {
                self.insert_tree(village, x, y);
            }
            for x in right as usize..TOWN_X {
                self.insert_tree(village, x, y);
            }
        }
        // A few single trees
        let n = lcg.next_in_range(0, 8);
        for _ in 0..n {
            let (x, y) = random_town_coordinate(lcg);
            if y as usize == TOWN_LANE_Y {
                continue;
            }
            if self.find_building_by_coordinates(x, y, village).is_none() {
                self.insert_tree(village, x as usize, y as usize);
            }
        }
    }

    fn insert_tree(&self, village: VillageKey, x: usize, y: usize) {
        let tree = BuildingFactory::new(BuildingType::Tree, (x, y), village);
        self.insert_building(&tree);
    }

    fn random_empty_town_coordinate(
        &self,
        village: VillageKey,
        lcg: &mut Lcg,
    ) -> Result<(i32, i32), &'static str> {
        for _ in 0..100 {
            let (x, y) = random_town_coordinate(lcg);
            if y as usize == TOWN_LANE_Y {
                continue;
            }
            if self.find_building_by_coordinates(x, y, village).is_none() {
                return Ok((x, y));
            }
        }
        Err("Didn't find an empty town slot")
    }

    fn generate_anarchist_hobos(
        &self,
        n: usize,
        village: VillageKey,
        lcg: &mut Lcg,
    ) -> Result<(), &'static str> {
        let hurried = false;
        let speed = 0.1;
        for _ in 0..n {
            let (x, y) = self.random_empty_town_coordinate(village, lcg)?;
            let hp = lcg.next_in_range(4, 6) as i64;
            self.insert_anarchist_hobo_with_nest(village, x, y, hp, speed, hurried);
        }
        Ok(())
    }

    fn insert_anarchist_hobo_with_nest(
        &self,
        village: VillageKey,
        x: i32,
        y: i32,
        hp: i64,
        speed: f32,
        hurried: bool,
    ) {
        let nest =
            BuildingFactory::new(BuildingType::SingleNest, (x as usize, y as usize), village);
        let nest_id = self.insert_building(&nest).key();
        self.insert_hobo(&NewHobo {
            hp,
            home: village.num(),
            color: Some(UnitColor::Yellow),
            speed,
            hurried,
            nest: Some(nest_id.num()),
        });
    }
}

fn random_town_coordinate(lcg: &mut Lcg) -> (i32, i32) {
    let x = lcg.next_in_range(0, TOWN_X as u64);
    let y = lcg.next_in_range(0, TOWN_Y as u64);
    (x as i32, y as i32)
}
