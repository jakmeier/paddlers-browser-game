//! Integrates Paddlers with the quicksilver framework
//! 
//! Quicksilver provides three things that we use:
//!  * Drawing a the draw loop
//!  * Game logic updates in another loop
//!  * User input events
//! 
//! All this is glued together by implementing quicksilver's State

use crate::prelude::*;
use crate::net::{
    game_master_api::RestApiState,
};
use crate::logging::{
    ErrorQueue,
    text_to_user::TextBoard,
};

use crate::game::*;
use crate::gui::ui_state::*;
use quicksilver::prelude::*;
use crate::specs::WorldExt;

use std::sync::Once;
static INIT: Once = Once::new();

impl State for Game<'static, 'static> {
    fn new() -> Result<Self> {
        Self::load_game()
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature="dev_view")]
        self.start_update();

        INIT.call_once(|| {
            self.initialize_with_window(window);
        });
        
        self.total_updates += 1;
        window.set_max_updates(1); // 1 update per frame is enough
        self.update_time_reference();
        self.pointer_manager.run(&mut self.world);
        {
            let now = self.world.read_resource::<Now>().0;
            let mut tick = self.world.write_resource::<ClockTick>();
            let us_draw_rate = 1_000_000/ 60;
            *tick = ClockTick((now / us_draw_rate) as u32);
        }
        {
            let mut q = self.world.write_resource::<ErrorQueue>();
            let mut t = self.world.write_resource::<TextBoard>();
            q.pull_async(&mut self.async_err_receiver, &mut t);
            q.run(&mut t);
        }
        if self.sprites.is_none() {
            self.update_loading(window)
        } else {
            self.main_update_loop(window)
        }?;
        #[cfg(feature="dev_view")]
        self.end_update();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature="dev_view")]
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
            let mut rest = self.world.write_resource::<RestApiState>();
            let err = self.stats.track_frame(&mut *rest, utc_now());
            self.check(err);
        }
        if self.sprites.is_none() {
            self.draw_loading(window)
        } else {
            self.draw_main(window)
        }?;
        #[cfg(feature="dev_view")]
        self.end_draw();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.handle_event(event, window)
    }
}