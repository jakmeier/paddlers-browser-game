//! Top-level view on game, as opposed to component-wise

use crate::prelude::*;
use paddle::*;

use crate::game::game_event_manager::load_game_event_manager;
use crate::game::*;
use crate::gui::ui_state::*;
use crate::init::{frame_loading::load_viewer, loading::PostInit};
use crate::net::graphql::{QuestsResponse, ReportsResponse};
use crate::specs::WorldExt;
#[derive(Clone, Debug)]
/// Signals are a way to broadcast events for event listeners across views.
pub enum Signal {
    ResourcesUpdated,
    PlayerInfoUpdated,
    LocaleUpdated,
    BuildingBuilt(BuildingType),
    BuildingUpgraded(BuildingType),
    BuildingRemoved(BuildingType),
    NewReportCount(usize),
    NewWorker(TaskType),
    WorkerStopped(TaskType),
}

impl Game {
    pub fn register<I>(initializer: I)
    where
        I: FnOnce(&mut Display) -> Game + 'static,
    {
        let fh = paddle::register_frame_with(
            GameActivity { initialized: false },
            move |display| initializer(display),
            (0, 0),
        );
        fh.listen(|_, game, _msg: &crate::init::loading::PostInit| {
            game.post_load().nuts_check();
        });
    }
}

struct GameActivity {
    /// For initialization of the game, which requires all data to be loaded previously and the Game objet to be placed in the domain.
    initialized: bool,
}
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

        if !self.initialized {
            self.initialize_game(game).nuts_check();
        }

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

impl GameActivity {
    fn initialize_game(&mut self, game: &mut Game) -> PadlResult<()> {
        let view = UiView::Town;
        let viewer = load_viewer(view);

        let mut loaded_data = game.loaded_data.take().unwrap();

        let leaderboard_data = *loaded_data.extract::<NetMsg>()?;
        paddle::share(leaderboard_data);

        let reports = *loaded_data.extract::<ReportsResponse>()?;
        paddle::share(NetMsg::Reports(reports));

        let quests = *loaded_data.extract::<QuestsResponse>()?;
        paddle::share(NetMsg::Quests(quests));

        let viewer_activity = nuts::new_domained_activity(viewer, &Domain::Frame);
        viewer_activity.subscribe_domained(|viewer, domain, _: &UpdateWorld| {
            let game: &mut Game = domain.try_get_mut().expect("Forgot to insert Game?");
            let view: UiView = *game.world.fetch();
            viewer.set_view(view);
        });
        load_game_event_manager();

        paddle::share_foreground(Signal::ResourcesUpdated);
        paddle::share(Signal::LocaleUpdated);

        crate::net::start_sync();
        paddle::share(PostInit);

        self.initialized = true;
        Ok(())
    }
}
