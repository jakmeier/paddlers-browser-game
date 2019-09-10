//! Uses a LCG to generate a pseudo-random sequence for the streams on the map
use paddlers_shared_lib::prelude::*;
use paddlers_shared_lib::game_mechanics::map::*;
use crate::db::DB;
mod lcg;
use lcg::Lcg;

struct NewMap {
    streams: Vec<NewStream>,
}

impl NewMap {
    /// Generates the map for a specific server (Seed = server id)
    fn generate(seed: u64) -> NewMap {
        let mut streams = vec![];
        let mut lcg = Lcg::new(seed);

        let start_y = 5.5;
        let dx = MAP_STREAM_AREA_W;
        for i in 0..4 {
            let b = (4 * i) as f32;
            streams.push(new_stream((b + 1.0, start_y), dx, 20.0, &mut lcg));
            streams.push(new_stream((b + 3.0, start_y), dx, -10.0, &mut lcg));
        }

        NewMap {
            streams
        }
    }
}

fn new_stream(start: (f32, f32), max_dx: f32, max_y: f32, lcg: &mut Lcg) -> NewStream {
    let mut control_points = vec![];

    let half_max_dx = max_dx / 2.0;
    let mut x = half_max_dx; // relative
    let mut y = start.1; // absolute
    let direction = if max_y > y { 1.0 } else { -1.0 };

    while direction * (max_y - y) > 0.0 {
        control_points.push(start.0 + x - half_max_dx);
        control_points.push(y);

        let dy = lcg.next_in_range(50, 150) as f32 / 100.0;
        y += dy * direction;

        let step_max_dx = half_max_dx.min(dy * 2.0);
        let min_x = (x - step_max_dx).max(0.0) * 100.0;
        let max_x = (x + step_max_dx).min(max_dx) * 100.0;
        x = lcg.next_in_range(min_x as u64, max_x as u64) as f32 / 100.0;

    }

    NewStream {
        start_x: start.0,
        control_points
    }
}

fn village_positions(stream_points: &[f32]) -> Vec<(f32,f32)> {
    let mut v : std::collections::HashSet<(i32,i32)> = std::collections::HashSet::new();
    let points: Vec<(f32,f32)> = stream_points.chunks_exact(2).map(|t|(t[0],t[1])).collect();
    let mut r = P(points[0].0, points[0].1);
    for slice in points.windows(2) {
        match slice {
            &[p,q] => {
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
                for t in 0 .. n {
                    let t = 1.0/n as f32 * t as f32;
                    let f = (p * (1.0-t) + r * t) * (1.0-t) + (r * (1.0-t) + q * t) * t;
                    let draw_anker = ((f.0-0.5).round(), (f.1 - 0.5).round());
                    let on_map = draw_anker.1 < MAP_H as f32
                                 && draw_anker.1 >= 0.0;
                    let on_river = 
                        draw_anker.1 ==
                        (MAP_H -1) as f32 / 2.0;
                    let distance2 = 
                          (draw_anker.0 + 0.5 - f.0).powi(2) 
                        + (draw_anker.1 + 0.5 - f.1).powi(2);
                    // defines radius of circle around center
                    let distance_close_enough = distance2 < 0.15; 
                    if !on_river && distance_close_enough && on_map {
                        // Village indices are stored human-readable
                        v.insert((draw_anker.0 as i32 + 1, draw_anker.1 as i32 + 1));
                    }
                }
                r = o;
            }
            _ => {panic!()},
        }
    }
    v.drain().map(|(a,b)|(a as f32, b as f32)).collect()
}

impl DB {
    pub fn init_map(&self, seed: u64) {
        let map = NewMap::generate(seed);
        
        self.insert_streams(&map.streams);
        // #[cfg(debug_assertions)]
        // self.test_add_all_villages();
    }

    pub fn add_village(&self) -> Result<Village, &'static str> {
        // Find unsaturated stream
        let streams = self.streams_to_add_village();
        for s in streams {
            // Pick a position on it
            let vp = village_positions(&s.control_points);
            for (x,y) in vp {
                if self.map_position_empty(x, y) {
                    let v = NewVillage {
                        stream_id: s.id,
                        x,
                        y,
                    };
                    return Ok(self.insert_villages(&[v])[0]);
                }
            }
        }
        Err("No space for another village")
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
            for (x,y) in vp {
                let v = NewVillage {
                    stream_id: s.id,
                    x,
                    y,
                };
                self.insert_villages(&[v]);
            }
        }
    }
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