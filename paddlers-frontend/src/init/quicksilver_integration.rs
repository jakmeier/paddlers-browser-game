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
use std::sync::mpsc::Receiver;

use crate::game::*;
use crate::game::town::Town;
use crate::gui::ui_state::*;
use crate::gui::input::pointer::PointerManager;
use quicksilver::prelude::*;
use crate::specs::WorldExt;
use crate::net::NetMsg;
use crate::Framer;

use std::sync::Once;
static INIT: Once = Once::new();

// TODO: reshuffle names, fix lifetimes
pub (crate) struct QuicksilverState {
    pub game: Game<'static, 'static>,
    pub pointer_manager: PointerManager<'static, 'static>,
    pub viewer: Framer,
}
impl QuicksilverState {
    pub fn load(resolution: ScreenResolution, net_chan: Receiver<NetMsg>) -> Self {

        let (game, ep) = Game::load_game().expect("Loading game");
        let mut game = game.with_town(Town::new(resolution))
            .with_resolution(resolution)
            .with_network_chan(net_chan);

        let pm = PointerManager::init(&mut game.world, ep.clone());
        let viewer = super::frame_loading::load_viewer(&mut game, ep);

        QuicksilverState {
            game,
            viewer,
            pointer_manager: pm,
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
        let err = self.viewer.set_view(view, &mut self.game);
        self.game.check(err);
        let err = self.viewer.update(&mut self.game);
        self.game.check(err);
        
        self.game.total_updates += 1;
        window.set_max_updates(1); // 1 update per frame is enough
        self.game.update_time_reference();
        self.pointer_manager.run(&mut self.game, &mut self.viewer);
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
            let err = self.viewer.draw(&mut self.game, window);
            self.game.check(err);
            let err = self.game.draw_main(window);
            self.game.check(err);
        }
        #[cfg(feature="dev_view")]
        self.game.end_draw();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        // TODO: position handling
        let err = self.viewer.event(&mut self.game, event);
        self.game.check(err);
        self.game.handle_event(event, window, &mut self.pointer_manager)
    }
}