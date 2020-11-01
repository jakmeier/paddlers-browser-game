use crate::net::game_master_api::RestApiState;
use chrono::{Duration, NaiveDateTime};
use paddlers_shared_lib::api::statistics::*;
use wasm_bindgen::prelude::wasm_bindgen;

const INTERVAL_SECONDS: i64 = 10;

pub struct Statistician {
    frames: i32,
    last_sent: NaiveDateTime,
    session_start: NaiveDateTime,
}

impl Statistician {
    pub fn new(now: NaiveDateTime) -> Self {
        Statistician {
            frames: 0,
            last_sent: now,
            session_start: now,
        }
    }

    // /// Call this once per frame to keep track of FPS and occasionally log data back to server
    // pub fn track_frame(&mut self, rest: &mut RestApiState, now: NaiveDateTime) {
    //     self.frames += 1;
    //     if self.last_sent + Duration::seconds(INTERVAL_SECONDS) < now {
    //         self.send(rest, now);
    //         self.last_sent = now;
    //         self.frames = 0;
    //     }
    // }

    fn send(&mut self, rest: &mut RestApiState, now: NaiveDateTime) {
        let interval_us = now - self.last_sent;
        let fps = 1_000_000.0 * self.frames as f64 / interval_us.num_microseconds().unwrap() as f64;
        let duration_us = now - self.session_start;
        let msg = FrontendRuntimeStatistics {
            browser: browser_info(),
            session_duration_s: duration_us.num_seconds(),
            fps: fps,
        };
        rest.http_send_statistics(msg)
    }
}

fn browser_info() -> BrowserInfo {
    JsBrowserInfo::new().into_serde().unwrap()
}

#[wasm_bindgen(module = "/src/logging/statistics.js")]
extern "C" {
    type JsBrowserInfo;

    #[wasm_bindgen(constructor)]
    fn new() -> JsBrowserInfo;
}
