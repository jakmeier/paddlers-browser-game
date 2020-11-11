//! Integrates Paddlers with the quicksilver framework
//!
//! Quicksilver provides three things that we use:
//!  * Drawing a the draw loop
//!  * Game logic updates in another loop
//!  * User input events
//!
//! All this is glued together by implementing quicksilver's State

use crate::prelude::*;
use paddle::web_integration::{start_drawing_thread, start_thread, ThreadHandler};
use paddle::*;

use crate::game::*;
use crate::gui::ui_state::*;
use crate::specs::WorldExt;

#[derive(Clone, Debug)]
/// Signals are a way to broadcast events for event listeners across views.
pub enum Signal {
    ResourcesUpdated,            // Notification
    PlayerInfoUpdated,           // Notification
    BuildingBuilt(BuildingType), // Signal
    NewReportCount(usize),       // Notification
}

// TODO: send events through nuts
// impl State for QuicksilverState {
//      ...
//     fn event(&mut self, event: &Event, window: &mut WebGLCanvas) -> Result<()> {
//         match self {
//             Self::Empty => {}
//             Self::Loading(_state) => {}
//             Self::Ready => nuts::publish(WorldEvent::new(window, event)),
//         }
//         Ok(())
//     }
// }

pub fn start_drawing() -> PadlResult<ThreadHandler> {
    Ok(start_drawing_thread(|t| nuts::publish(DrawWorld::new(t)))?)
}

pub fn start_updating() -> PadlResult<ThreadHandler> {
    Ok(start_thread(|| nuts::publish(UpdateWorld::new()), 10)?)
}

struct GameActivity;
impl Game {
    pub fn register_in_nuts() {
        let aid = nuts::new_domained_activity(GameActivity, &Domain::Frame);
        aid.subscribe_domained_mut(|_, domain, _msg: &mut UpdateWorld| {
            let game: &mut Game = domain.try_get_mut().expect("Game missing");
            if let Err(e) = game.update() {
                let err: PadlError = e.into();
                nuts::publish(err);
            }
        });
        aid.subscribe_domained(|_, domain, _msg: &DrawWorld| {
            let (game, window) = domain.try_get_2_mut::<Game, WebGLCanvas>();
            let (game, window) = (game.expect("Game missing"), window.expect("Window missing"));
            if let Err(e) = game.draw(window) {
                let err: PadlError = e.into();
                nuts::publish(err);
            }
        });
    }
    fn update(&mut self) -> PadlResult<()> {
        #[cfg(feature = "dev_view")]
        self.start_update();

        self.total_updates += 1;
        self.update_time_reference();

        {
            let now = self.world.read_resource::<Now>().0;
            let mut tick = self.world.write_resource::<ClockTick>();
            let ms_draw_rate = 1_000 / 60;
            *tick = ClockTick((now.timestamp_millis() / ms_draw_rate) as u32);
        }

        let res = self.update_net();
        self.check(res);
        self.main_update_loop()?;

        #[cfg(feature = "dev_view")]
        self.end_update();
        Ok(())
    }

    fn draw(&mut self, window: &mut WebGLCanvas) -> PadlResult<()> {
        #[cfg(feature = "dev_view")]
        self.start_draw();

        // TODO (optimization): Refactor to make this call event-based
        if self.total_updates % 50 == 0 {
            let err = crate::window::adapt_window_size(window);
            self.check(err);
        }

        // Probably just remove
        // {
        //     let mut rest = RestApiState::get();
        //     let err = self.stats.track_frame(&mut *rest, utc_now());
        //     self.check(err);
        // }

        #[cfg(feature = "dev_view")]
        self.draw_dev_view(window);

        #[cfg(feature = "dev_view")]
        self.end_draw();

        Ok(())
    }
}
