//! Uses a LCG to generate a pseudo-random sequence for the streams on the map

mod village_creation;

use crate::db::DB;
use paddlers_shared_lib::game_mechanics::map::*;
use paddlers_shared_lib::prelude::*;
mod lcg;
use lcg::Lcg;

const STREAMS: usize = 100;
const ANARCHISTS: usize = 100;

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
        for i in 0..STREAMS {
            let b = (4 * i) as f32;
            streams.push(new_stream((b + 1.0, start_y), dx, 20.0, &mut lcg));
            streams.push(new_stream((b + 3.0, start_y), dx, -10.0, &mut lcg));
        }

        NewMap { streams }
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
        control_points,
    }
}

impl DB {
    pub fn init_map(&self, seed: u64) {
        let map = NewMap::generate(seed);

        self.insert_streams(&map.streams);

        if let Err(e) = self.generate_anarchists(ANARCHISTS, seed.wrapping_add(1)) {
            eprintln!("Failure on anarchists spawning: {}", e);
        }
        // #[cfg(debug_assertions)]
        // self.test_add_all_villages();
    }
}
