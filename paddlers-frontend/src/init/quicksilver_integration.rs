//! Integrates Paddlers with the quicksilver framework
//!
//! Quicksilver provides three things that we use:
//!  * Drawing a the draw loop
//!  * Game logic updates in another loop
//!  * User input events
//!
//! All this is glued together by implementing quicksilver's State

use crate::prelude::*;
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

struct GameActivity;
impl Frame for GameActivity {
    type Error = PadlError;
    type State = Game;

    fn draw(
        &mut self,
        game: &mut Self::State,
        canvas: &mut WebGLCanvas,
        _timestamp: f64,
    ) -> Result<(), Self::Error> {
        game.draw(canvas)
    }
    fn update(&mut self, game: &mut Self::State) -> Result<(), Self::Error> {
        game.update()
    }
}
impl Game {
    pub fn register(self) {
        nuts::store_to_domain(&Domain::Frame, self);
        paddle::frame_to_activity(GameActivity, &Domain::Frame);
    }
    fn update(&mut self) -> PadlResult<()> {
        // TODO: Time tracking like this does not work anymore. Probably add to paddle?
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
