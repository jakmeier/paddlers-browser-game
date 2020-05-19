use crate::buildings::BuildingFactory;
use crate::db::DB;
use crate::setup::map_generation::Lcg;
use paddlers_shared_lib::game_mechanics::map::*;
use paddlers_shared_lib::game_mechanics::town::*;
use paddlers_shared_lib::prelude::*;

impl DB {
    pub fn add_village(&self, pid: PlayerKey) -> Result<Village, &'static str> {
        // Find unsaturated stream
        let streams = self.streams_to_add_village();
        for s in &streams {
            // Pick a position on it
            match self.insert_village_on_stream(s, Some(pid)) {
                Err(_) => {}
                Ok(v) => return Ok(v),
            }
        }
        Err("World full: No space for another village")
    }
    pub fn generate_anarchists(&self, n: usize, seed: u64) -> Result<(), &'static str> {
        let mut lcg = Lcg::new(seed);
        for i in 1..n + 1 {
            self.add_anarchists_village(StreamKey(i as i64), &mut lcg)?;
        }
        Ok(())
    }
    fn add_anarchists_village(
        &self,
        stream_id: StreamKey,
        lcg: &mut Lcg,
    ) -> Result<Village, &'static str> {
        let s = self.stream(stream_id);
        let village = self.insert_village_on_stream(&s, None)?;
        self.add_random_forest_to_village(village.key(), lcg);
        Ok(village)
    }
    fn insert_village_on_stream(
        &self,
        s: &Stream,
        player: Option<PlayerKey>,
    ) -> Result<Village, &'static str> {
        let vp = village_positions(&s.control_points);
        for (x, y) in vp {
            if self.map_position_empty(x, y) {
                let v = NewVillage {
                    stream_id: s.id,
                    x,
                    y,
                    player_id: player.as_ref().map(PlayerKey::num),
                    faith: None, // Start with default value
                };
                return Ok(self.insert_villages(&[v])[0]);
            }
        }
        Err("Stream full: No space for another village")
    }

    fn streams_to_add_village(&self) -> Vec<Stream> {
        // Good enough for now
        self.streams(0.0, MAP_MAX_X as f32)
    }

    fn map_position_empty(&self, x: f32, y: f32) -> bool {
        self.village_at(x, y).is_none()
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
            let x = lcg.next_in_range(0, TOWN_X as u64);
            let y = lcg.next_in_range(0, TOWN_Y as u64);
            if y as usize == TOWN_LANE_Y {
                continue;
            }
            if self
                .find_building_by_coordinates(x as i32, y as i32, village)
                .is_none()
            {
                self.insert_tree(village, x as usize, y as usize);
            }
        }
    }
    fn insert_tree(&self, village: VillageKey, x: usize, y: usize) {
        let tree = BuildingFactory::new(BuildingType::Tree, (x, y), village);
        self.insert_building(&tree);
    }

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn test_add_all_villages(&self) {
        let streams = self.streams(-1.0, 21.0);
        for s in streams {
            let vp = village_positions(&s.control_points);
            for (x, y) in vp {
                let v = NewVillage {
                    stream_id: s.id,
                    x,
                    y,
                    player_id: None,
                    faith: None, // Start with default value
                };
                self.insert_villages(&[v]);
            }
        }
    }
}

fn village_positions(stream_points: &[f32]) -> Vec<(f32, f32)> {
    let mut v: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let points: Vec<(f32, f32)> = stream_points
        .chunks_exact(2)
        .map(|t| (t[0], t[1]))
        .collect();
    let mut r = P(points[0].0, points[0].1);
    for slice in points.windows(2) {
        match slice {
            &[p, q] => {
                let p = P(p.0, p.1);
                let q = P(q.0, q.1);
                /* p,q are bezier control points
                 * their center define the fixed point on the curve
                 * for the previous pair of control points (o,p)
                 */
                let o = P((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
                /* formula:
                 * f(0 <= t <= 1) =
                 *     (1-t)[(1-t)p + t*r]
                 *   + (t)  [(1-t)r + t*q]
                 */

                let n = 4;
                for t in 0..n {
                    let t = 1.0 / n as f32 * t as f32;
                    let f = (p * (1.0 - t) + r * t) * (1.0 - t) + (r * (1.0 - t) + q * t) * t;
                    let draw_anker = ((f.0 - 0.5).round(), (f.1 - 0.5).round());
                    let on_map = draw_anker.1 < MAP_H as f32 && draw_anker.1 >= 0.0;
                    let on_river = draw_anker.1 == (MAP_H - 1) as f32 / 2.0;
                    let distance2 =
                        (draw_anker.0 + 0.5 - f.0).powi(2) + (draw_anker.1 + 0.5 - f.1).powi(2);
                    // defines radius of circle around center
                    let distance_close_enough = distance2 < 0.15;
                    if !on_river && distance_close_enough && on_map {
                        // Village indices are stored human-readable
                        v.insert((draw_anker.0 as i32 + 1, draw_anker.1 as i32 + 1));
                    }
                }
                r = o;
            }
            _ => panic!(),
        }
    }
    v.drain().map(|(a, b)| (a as f32, b as f32)).collect()
}

use core::ops::*;
#[derive(Copy, Clone, Debug)]
struct P(f32, f32);
impl Mul<f32> for P {
    type Output = P;
    fn mul(self, rhs: f32) -> P {
        P(self.0 * rhs, self.1 * rhs)
    }
}
impl Add for P {
    type Output = P;
    fn add(self, rhs: P) -> P {
        P(self.0 + rhs.0, self.1 + rhs.1)
    }
}
