//! Integrates Paddlers with the quicksilver framework
//!
//! Quicksilver provides three things that we use:
//!  * Drawing a the draw loop
//!  * Game logic updates in another loop
//!  * User input events
//!
//! All this is glued together by implementing quicksilver's State

use crate::init::loading::LoadingState;
use crate::net::game_master_api::RestApiState;
use crate::prelude::*;
use paddle::utc_now;
use paddle::*;

use crate::game::*;
use crate::gui::ui_state::*;
use crate::specs::WorldExt;
use quicksilver::prelude::*;

use std::sync::Once;
static INIT: Once = Once::new();

pub(crate) enum QuicksilverState {
    /// Used for easy data swapping
    Empty,
    // While downloading resources
    Loading(LoadingState),
    // During fully initialized game (Game is stored in nuts)
    Ready,
}
#[derive(Clone, Debug)]
/// Signals are a way to broadcast events for event listeners across views.
pub enum Signal {
    ResourcesUpdated,            // Notification
    PlayerInfoUpdated,           // Notification
    BuildingBuilt(BuildingType), // Signal
    NewReportCount(usize),       // Notification
}

impl QuicksilverState {
    pub fn load(state: LoadingState) -> Self {
        Self::Loading(state)
    }
}
impl State for QuicksilverState {
    fn new() -> Result<Self> {
        unreachable!()
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        match self {
            Self::Loading(state) => {
                let err = state.update_net();
                state.queue_error(err);
                self.try_finalize();
            }
            Self::Ready => nuts::publish(UpdateWorld::new(window)),

            Self::Empty => {
                println!("Fatal error: No state");
            }
        }
        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match self {
            Self::Loading(state) => {
                if let Err(e) = state.draw_loading(window) {
                    nuts::publish(e);
                }
            }
            Self::Ready => {
                nuts::publish(DrawWorld::new(window));
            }
            Self::Empty => {}
        }
        Ok(())
    }
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match self {
            Self::Empty => {}
            Self::Loading(_state) => {}
            Self::Ready => nuts::publish(WorldEvent::new(window, event)),
        }
        Ok(())
    }
}
struct GameActivity;
impl Game<'static, 'static> {
    pub fn register_in_nuts() {
        let aid = nuts::new_domained_activity(GameActivity, &Domain::Main);
        aid.subscribe_domained_mut(|_, domain, msg: &mut UpdateWorld| {
            let game: &mut Game = domain.try_get_mut().expect("Game missing");
            let window = msg.window();
            if let Err(e) = game.update(window) {
                let err: PadlError = e.into();
                nuts::publish(err);
            }
        });
        aid.subscribe_domained_mut(|_, domain, msg: &mut DrawWorld| {
            let game: &mut Game = domain.try_get_mut().expect("Game missing");
            let window = msg.window();
            if let Err(e) = game.draw(window) {
                let err: PadlError = e.into();
                nuts::publish(err);
            }
        });
    }
    fn update(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature = "dev_view")]
        self.start_update();

        INIT.call_once(|| {
            self.initialize_with_window(window);
        });

        self.total_updates += 1;
        window.set_max_updates(1); // 1 update per frame is enough
        self.update_time_reference();

        {
            let now = self.world.read_resource::<Now>().0;
            let mut tick = self.world.write_resource::<ClockTick>();
            let ms_draw_rate = 1_000 / 60;
            *tick = ClockTick((now.timestamp_millis() / ms_draw_rate) as u32);
        }

        let res = self.update_net();
        self.check(res);
        self.main_update_loop(window)?;

        #[cfg(feature = "dev_view")]
        self.end_update();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature = "dev_view")]
        self.start_draw();

        INIT.call_once(|| {
            self.initialize_with_window(window);
        });

        // TODO (optimization): Refactor to make this call event-based
        if self.total_updates % 50 == 0 {
            let err = crate::window::adapt_window_size(window);
            self.check(err);
        }

        {
            let mut rest = RestApiState::get();
            let err = self.stats.track_frame(&mut *rest, utc_now());
            self.check(err);
        }

        #[cfg(feature = "dev_view")]
        self.draw_dev_view(window);

        #[cfg(feature = "dev_view")]
        self.end_draw();

        Ok(())
    }
}
