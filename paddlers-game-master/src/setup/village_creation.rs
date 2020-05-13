use crate::db::DB;
use paddlers_shared_lib::game_mechanics::map::*;
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
    pub fn generate_anarchists(&self, n: usize) -> Result<(), &'static str> {
        for i in 1..n + 1 {
            self.add_anarchists_village(StreamKey(i as i64))?;
        }
        Ok(())
    }
    fn add_anarchists_village(&self, stream_id: StreamKey) -> Result<Village, &'static str> {
        let s = self.stream(stream_id);
        self.insert_village_on_stream(&s, None)
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
