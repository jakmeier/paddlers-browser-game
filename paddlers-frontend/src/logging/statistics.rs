use crate::net::game_master_api::RestApiState;
use crate::prelude::*;
use paddlers_shared_lib::api::statistics::*;
use stdweb::unstable::TryInto;

const INTERVAL_SECONDS: i64 = 10;

pub struct Statistician {
    frames: i32,
    last_sent: Timestamp,
    session_start: Timestamp,
}

impl Statistician {
    pub fn new(now: Timestamp) -> Self {
        Statistician {
            frames: 0,
            last_sent: now,
            session_start: now,
        }
    }

    /// Call this once per frame to keep track of FPS and occasionally log data back to server
    pub fn track_frame(&mut self, rest: &mut RestApiState, now: Timestamp) -> PadlResult<()> {
        self.frames += 1;
        if self.last_sent + INTERVAL_SECONDS * 1_000_000 < now {
            self.send(rest, now)?;
            self.last_sent = now;
            self.frames = 0;
        }
        Ok(())
    }

    fn send(&mut self, rest: &mut RestApiState, now: Timestamp) -> PadlResult<()> {
        let interval_us = now - self.last_sent;
        let fps = 1_000_000.0 * self.frames as f64 / interval_us as f64;
        let duration_us = now - self.session_start;
        let msg = FrontendRuntimeStatistics {
            browser: browser_info(),
            session_duration_s: duration_us / 1_000_000,
            fps: fps,
        };
        rest.http_send_statistics(msg)
    }
}

fn browser_info() -> BrowserInfo {
    let user_agent: String = js!(
        return navigator.userAgent;
    )
    .try_into()
    .unwrap_or("NotAvailable".to_owned());

    let window = stdweb::web::window();
    BrowserInfo {
        user_agent: user_agent,
        inner_width: window.inner_width(),
        inner_height: window.inner_height(),
        outer_width: window.outer_width(),
        outer_height: window.outer_height(),
    }
}
