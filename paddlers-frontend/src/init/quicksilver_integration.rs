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
use crate::gui::input::UiView;
use quicksilver::prelude::*;
use crate::specs::WorldExt;
use crate::view::FrameManager;
use crate::gui::menu::MenuFrame;

use std::sync::Once;
static INIT: Once = Once::new();

// TODO: reshuffle names
pub (crate) struct QuicksilverState {
    pub game: Game<'static, 'static>,
    pub viewer: FrameManager<UiView, Game<'static, 'static>, Window, PadlError>,
}
impl QuicksilverState {
    pub fn load(game: Game<'static,'static>) -> Self {
        let mut viewer: FrameManager<UiView,Game<'static,'static>,Window,PadlError> = Default::default();
        let menu = MenuFrame::new().expect("Menu loading");
        viewer.add_frame(
            Box::new(menu),
            &[UiView::Attacks, UiView::Town, UiView::Leaderboard, UiView::Map],
            (0,0), // TODO
            (0,0), // TODO
        );
        QuicksilverState {
            game,
            viewer,
        }
    }
}
impl State for QuicksilverState {
    fn new() -> Result<Self> {
        unreachable!()
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature="dev_view")]
        self.game.start_update();

        INIT.call_once(|| {
            self.game.initialize_with_window(window);
        });

        let view = self.game.world.fetch::<UiState>().current_view;
        self.viewer.set_view(view, &mut self.game);
        self.viewer.update(&mut self.game);
        
        self.game.total_updates += 1;
        window.set_max_updates(1); // 1 update per frame is enough
        self.game.update_time_reference();
        self.game.pointer_manager.run(&mut self.game.world);
        {
            let now = self.game.world.read_resource::<Now>().0;
            let mut tick = self.game.world.write_resource::<ClockTick>();
            let us_draw_rate = 1_000_000/ 60;
            *tick = ClockTick((now / us_draw_rate) as u32);
        }
        {
            let mut q = self.game.world.write_resource::<ErrorQueue>();
            let mut t = self.game.world.write_resource::<TextBoard>();
            q.pull_async(&mut self.game.async_err_receiver, &mut t);
            q.run(&mut t);
        }
        if self.game.sprites.is_none() {
            self.game.update_loading(window)
        } else {
            self.game.main_update_loop(window)
        }?;
        #[cfg(feature="dev_view")]
        self.game.end_update();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        #[cfg(feature="dev_view")]
        self.game.start_draw();

        INIT.call_once(|| {
            self.game.initialize_with_window(window);
        });

        // TODO (optimization): Refactor to make this call event-based
        if self.game.total_updates % 50 == 0 {
            let err = crate::window::adapt_window_size(window);
            self.game.check(err);
        }

        {
            let mut rest = self.game.world.write_resource::<RestApiState>();
            let err = self.game.stats.track_frame(&mut *rest, utc_now());
            self.game.check(err);
        }
        if self.game.sprites.is_none() {
            let res = self.game.draw_loading(window);
            self.game.check(res);
        } else {
            window.clear(Color::WHITE)?;
            self.viewer.draw(&mut self.game, window);
            let res = self.game.draw_main(window);
            self.game.check(res);
        }
        #[cfg(feature="dev_view")]
        self.game.end_draw();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.game.handle_event(event, window)
    }
}