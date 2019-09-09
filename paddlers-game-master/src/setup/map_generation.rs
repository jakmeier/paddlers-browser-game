//! Uses a LCG to generate a pseudo-random sequence for the streams on the map
use paddlers_shared_lib::prelude::*;
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
        streams.push(new_stream((1.0, start_y), 5.0, 20.0, &mut lcg));
        streams.push(new_stream((3.0, start_y), 5.0, -10.0, &mut lcg));
        streams.push(new_stream((5.0, start_y), 5.0, 20.0, &mut lcg));
        streams.push(new_stream((7.0, start_y), 5.0, -10.0, &mut lcg));
        streams.push(new_stream((9.0, start_y), 5.0, 20.0, &mut lcg));
        streams.push(new_stream((11.0, start_y), 5.0, -10.0, &mut lcg));
        streams.push(new_stream((13.0, start_y), 5.0, 20.0, &mut lcg));

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
    // TODO
    let mut v = vec![];
    let points: Vec<(f32,f32)> = stream_points.chunks_exact(2).map(|t|(t[0],t[1])).collect();
    for slice in points.windows(2) {
        match slice {
            &[p,q] => {
                let r = (p.0 + q.0 / 2.0, p.1 + q.1 / 2.0);
                v.push((r.0.round(), r.1.round()));
            }
            _ => {panic!()},
        }
    }
    v
}

impl DB {
    pub fn init_map(&self, seed: u64) {
        let map = NewMap::generate(seed);
        
        self.insert_streams(&map.streams);
    }

    pub fn add_village(&self) -> Village {
        // TODO
        // Find unsaturated stream
        let streams = self.streams(0.0, 20.0); // TODO
        let s = &streams[0]; // TODO
        // Pick a position on it
        let n = self.all_villages().len();
        let vp = village_positions(&s.control_points); // TODO
        let (x,y) = vp[vp.len()-n-1]; // TODO
        let v = NewVillage {
            stream_id: s.id,
            x,
            y,
        };
        self.insert_villages(&[v])[0]
    }
}