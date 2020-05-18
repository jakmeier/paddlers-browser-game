//! Runtime performance tests

use crate::game::Game;
use crate::prelude::*;
use crate::specs::WorldExt;

mod standard_village;

pub(crate) struct TestData {
    pub kind: Test,
    start: Timestamp,
    end: Timestamp,
    total_frames: i32,
    intervals: Vec<Timestamp>,
    prev_interval: Timestamp,
    current_frame_start: Timestamp,
    current_update_start: Timestamp,
    draw_intervals: Vec<Timestamp>,
    update_intervals: Vec<Timestamp>,
    draw_frame_target_us: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Test {
    Vanilla,
    Empty,
    StandardVillage,
}

impl TestData {
    pub fn start_test(game: &mut Game<'_, '_>, setting: Test) -> Self {
        let resolution = *game.world.fetch::<ScreenResolution>();
        match setting {
            Test::Vanilla => { /* NOP */ }
            s => {
                game.flush_hobos().expect("Flushing hobos");
                game.flush_buildings().expect("Flushing buildings");
                match s {
                    Test::Vanilla => unreachable!(),
                    Test::Empty => { /* NOP */ }
                    Test::StandardVillage => {
                        standard_village::insert_hobos(game.town_world_mut(), resolution)
                            .expect("inserting test hobos")
                    }
                }
                let mut town = game.world.fetch_mut();
                let entities = game.world.entities();
                let lazy = game.world.fetch();
                match s {
                    Test::Vanilla => unreachable!(),
                    Test::Empty => { /* NOP */ }
                    Test::StandardVillage => {
                        standard_village::insert_buildings(&mut town, &entities, &lazy)
                    }
                }
            }
        }
        game.world.maintain();

        println!("Starting {:?} test", setting);
        let now = utc_now();
        let dt_seconds = chrono::Duration::seconds(10);
        TestData {
            kind: setting,
            start: now,
            end: now + dt_seconds,
            total_frames: 0,
            intervals: vec![],
            draw_intervals: vec![],
            update_intervals: vec![],
            prev_interval: now,
            current_frame_start: Timestamp::from_us(-1),
            current_update_start: Timestamp::from_us(-1),
            draw_frame_target_us: 10_000,
        }
    }
    pub fn record_start_of_update(&mut self) {
        self.current_update_start = utc_now();
    }
    pub fn record_end_of_update(&mut self) {
        if self.current_update_start.micros() > 0 {
            let now = utc_now();
            let dt = now - self.current_update_start;
            self.update_intervals.push(dt);
        }
    }
    pub fn record_start_of_frame(&mut self) {
        self.current_frame_start = utc_now();
    }
    pub fn record_end_of_frame(&mut self) {
        self.total_frames += 1;
        let now = utc_now();
        let dt = now - self.prev_interval;
        self.prev_interval = now;
        self.intervals.push(dt);
        if self.current_frame_start.micros() > 0 {
            let draw_dt = now - self.current_frame_start;
            self.draw_intervals.push(draw_dt);
        }
    }
    pub fn result(&self) -> Option<String> {
        let now = utc_now();
        if self.end < now {
            Some(self.evaluate())
        } else {
            None
        }
    }
    pub fn evaluate(&self) -> String {
        let dt = self.end - self.start;
        let fps = self.total_frames as f64 * 1e6 / dt.micros() as f64;
        let avg = statistical::mean(
            &self
                .intervals
                .iter()
                .map(|i| i.micros() as f64 / 1000.0)
                .collect::<Vec<_>>(),
        );

        let (min, max, median) = min_max_median(&self.intervals);
        let (draw_min, draw_max, draw_median) = min_max_median(&self.draw_intervals);
        let (update_min, update_max, update_median) = min_max_median(&self.update_intervals);

        let missed = self
            .draw_intervals
            .iter()
            .filter(|dt| dt.micros() > self.draw_frame_target_us)
            .fold(0, |acc, _| acc + 1);
        let missed = 100.0 * missed as f64 / self.draw_intervals.len() as f64;

        format!("{:.02}FPS = {:.02}ms, {:.02}ms {:.02}ms {:.02}ms |DRAW| {:.02}ms {:.02}ms {:.02}ms, {:.03}% missed |UPDATE| {:.02}ms {:.02}ms {:.02}ms",
            fps, avg, min, median, max,
            draw_min, draw_median, draw_max, missed,
            update_min, update_median, update_max,
        )
    }
}

fn min_max_median(data: &[Timestamp]) -> (f64, f64, f64) {
    let data = data.iter().map(Timestamp::micros).collect::<Vec<_>>();
    let min = data.iter().fold(std::i64::MAX, |acc, i| acc.min(*i)) as f64 / 1000.0;
    let max = data.iter().fold(std::i64::MIN, |acc, i| acc.max(*i)) as f64 / 1000.0;
    let median = statistical::median(&data) as f64 / 1000.0;
    (min, max, median)
}
