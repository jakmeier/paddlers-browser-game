//! Top-level view on game, as opposed to component-wise

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

impl Game {
    pub fn register(self) {
        let fh = paddle::register_frame(GameActivity, self, (0, 0));
        fh.listen(|_, game, _msg: &crate::init::loading::PostInit| {
            game.post_load().nuts_check();
        });
    }
}

struct GameActivity;
impl Frame for GameActivity {
    type State = Game;
    const WIDTH: u32 = crate::resolution::SCREEN_W;
    const HEIGHT: u32 = crate::resolution::SCREEN_H;

    fn draw(&mut self, game: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        #[cfg(feature = "dev_view")]
        game.start_draw();

        // TODO (optimization): Refactor to make this call event-based
        if game.total_updates % 31 == 0 {
            window.fit_display(10.0);
        }

        #[cfg(feature = "dev_view")]
        game.draw_dev_view(window);

        #[cfg(feature = "dev_view")]
        game.end_draw();
    }
    fn update(&mut self, game: &mut Self::State) {
        // TODO: Time tracking like this does not work anymore. Probably add to paddle?
        #[cfg(feature = "dev_view")]
        game.start_update();

        game.total_updates += 1;
        game.update_time_reference();

        {
            let now = game.world.read_resource::<Now>().0;
            let mut tick = game.world.write_resource::<ClockTick>();
            let ms_draw_rate = 1_000 / 60;
            *tick = ClockTick((now.timestamp_millis() / ms_draw_rate) as u32);
        }

        let res = game.update_net();
        game.check(res);
        game.main_update_loop().nuts_check();

        #[cfg(feature = "dev_view")]
        game.end_update();
    }
    fn key(&mut self, state: &mut Self::State, key: KeyEvent) {
        if let KeyEvent(KeyEventType::KeyDown, key) = key {
            state.hotkey(key);
        }
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        state.mouse.track_pointer_event(&event);
    }
}
