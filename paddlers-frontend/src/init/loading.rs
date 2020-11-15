use crate::net::graphql::{
    query_types::{
        AttacksResponse, BuildingsResponse, HobosQueryResponse, VolatileVillageInfoResponse,
    },
    ReportsResponse,
};
use crate::{game::game_event_manager::load_game_event_manager, prelude::PadlError};
use crate::{game::net_receiver::loading_update_net, init::quicksilver_integration::Signal};
use crate::{gui::input::UiView, prelude::PadlErrorCode};
use nuts::LifecycleStatus;
use paddle::{
    graphics::Image, graphics::ImageLoader, ErrorMessage, Frame, LoadScheduler, LoadedData,
    LoadingDone, LoadingProgress, NutsCheck, TextBoard, UpdateWorld, WebGLCanvas,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::game::player_info::PlayerInfo;
use crate::game::Game;
use crate::gui::sprites::{
    animation::{AnimatedObject, AnimatedObjectDef, AnimationVariantDef},
    paths::{ANIMATION_DEFS, SPRITE_PATHS},
    Sprites,
};
use crate::gui::utils::*;
use crate::net::graphql::query_types::WorkerResponse;
use crate::net::NetMsg;
use crate::prelude::{PadlResult, ScreenResolution, TextDb};
use paddle::quicksilver_compat::*;
use paddle::{Domain, FloatingText, WorldEvent};
use std::sync::mpsc::Receiver;

/// State that is used while loading all data over the network.
/// It will automatically be removed when loading is done.
pub(crate) struct LoadingFrame {
    pub resolution: ScreenResolution,
    preload_float: FloatingText,
    net_chan: Receiver<NetMsg>,
}

pub struct GameLoadingData {
    pub player_info: PlayerInfo,
    pub worker_response: WorkerResponse,
    pub buildings_response: BuildingsResponse,
    pub hobos_response: HobosQueryResponse,
    pub attacking_hobos: AttacksResponse,
    pub village_info: VolatileVillageInfoResponse,
    pub reports: ReportsResponse,
}

impl LoadingFrame {
    fn run_as_activity(self) {
        let aid = paddle::frame_to_activity(self, &Domain::Frame);
        aid.subscribe_domained(|_loading_state, domain, msg: &LoadingProgress| {
            domain.store(Some(msg.clone()));
        });
        aid.on_delete_domained(|loading_state, domain| {
            let loaded_data = std::mem::take(domain.get_mut::<LoadedData>());
            loading_state.finalize(loaded_data).nuts_check();
        });
        aid.subscribe(move |_loading_state, _msg: &LoadingDone| {
            aid.set_status(LifecycleStatus::Deleted);
        });
    }
    pub fn start(
        resolution: ScreenResolution,
        root_id: &str,
        net_chan: Receiver<NetMsg>,
    ) -> PadlResult<()> {
        let document = div::doc()?;
        let root = document
            .get_element_by_id(root_id)
            .ok_or(PadlError::dev_err(PadlErrorCode::DevMsg(
                "Root element not found by id.",
            )))?;
        let canvas: HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(|_| "canvas creation failed")?
            .dyn_into()
            .unwrap();
        root.append_child(&canvas)?;
        Self::start_with_canvas(resolution, canvas, net_chan);
        Ok(())
    }
    pub fn start_with_canvas(
        resolution: ScreenResolution,
        canvas: HtmlCanvasElement,
        net_chan: Receiver<NetMsg>,
    ) {
        let config = paddle::PaddleConfig::default()
            .with_resolution(resolution.pixels())
            .with_canvas(canvas);
        paddle::init(config).expect("Failed creating window");

        let mut images = vec![];
        for src in &SPRITE_PATHS {
            let img = async move { Image::load(src).await };
            images.push(img);
        }
        let animations = start_loading_animations();
        let locale = start_loading_locale();

        let load_manager = LoadScheduler::new()
            .with_vec(images, "Drawing visuals for the game")
            .with_vec(animations, "Animating fellow Paddlers")
            .with(locale, "Writing localized texts")
            .with_manually_reported::<NetMsg>("Collecting news in Paddland")
            .with_manually_reported::<PlayerInfo>("Downloading player data")
            .with_manually_reported::<WorkerResponse>("Summon working Paddlers")
            .with_manually_reported::<BuildingsResponse>("Construct buildings")
            .with_manually_reported::<HobosQueryResponse>("Summon non-working Paddlers")
            .with_manually_reported::<AttacksResponse>("Summon visitors")
            .with_manually_reported::<VolatileVillageInfoResponse>("Gather village news")
            .with_manually_reported::<ReportsResponse>("Gather village news");

        load_manager.attach_to_domain();

        let preload_float = FloatingText::try_default().expect("FloatingText");

        LoadingFrame {
            resolution,
            preload_float,
            net_chan,
        }
        .run_as_activity();
    }

    fn draw_progress(
        &mut self,
        window: &mut WebGLCanvas,
        progress: f32,
        msg: &str,
    ) -> PadlResult<()> {
        // TODO (optimization): Refactor to make this call event-based
        crate::window::adapt_window_size(window)?;

        window.clear(DARK_GREEN);
        let r = self.resolution;
        let w = r.pixels().0;
        let y = r.progress_bar_area_y();
        let ph = r.progress_bar_area_h();
        let area = Rectangle::new((w * 0.1, y), (w * 0.8, ph));

        draw_progress_bar(window, &mut self.preload_float, area, progress, &msg)?;
        Ok(())
    }
    fn finalize(self, mut loaded_data: LoadedData) -> PadlResult<()> {
        let catalog = (*loaded_data.extract::<PadlResult<gettext::Catalog>>()?)?;
        let maybe_images: Vec<paddle::PaddleResult<Image>> = loaded_data.extract_vec()?;
        let mut images = vec![];
        for maybe_image in maybe_images {
            images.push(maybe_image?);
        }
        let maybe_animations: Vec<PadlResult<AnimatedObject>> = loaded_data.extract_vec()?;
        let mut animations = vec![];
        for maybe_animation in maybe_animations {
            animations.push(maybe_animation?);
        }

        let (resolution, net_chan) = (self.resolution, self.net_chan);
        let sprites = Sprites::new(images, animations);

        let game_data = GameLoadingData::try_from_loaded_data(&mut loaded_data)?;

        let leaderboard_data = *loaded_data.extract::<NetMsg>()?;

        match Game::load_game(sprites, catalog, resolution, game_data, net_chan) {
            Err(e) => {
                TextBoard::display_error_message(":(\nLoading game failed".to_owned()).nuts_check(); // TODO: multi-lang errors
                panic!("Fatal Error: Could not load game {:?}", e);
            }
            Ok(mut game) => {
                let pointer_manager =
                    crate::gui::input::pointer::PointerManager::init(&mut game.world);
                game.register();
                let view = UiView::Town;
                let viewer = super::frame_loading::load_viewer(view, resolution);
                paddle::share(leaderboard_data);
                paddle::share_foreground(Signal::ResourcesUpdated);

                let viewer_activity = nuts::new_domained_activity(viewer, &Domain::Frame);
                viewer_activity.subscribe_domained(|viewer, domain, _: &UpdateWorld| {
                    let game: &mut Game = domain.try_get_mut().expect("Forgot to insert Game?");
                    // FIXME; really need to be set every frame?
                    let view: UiView = *game.world.fetch();
                    viewer.set_view(view);
                });

                let pointer_manager_activity =
                    nuts::new_domained_activity(pointer_manager, &Domain::Frame);
                pointer_manager_activity.subscribe_domained(
                    |pointer_manager, domain, _: &UpdateWorld| {
                        let game: &mut Game = domain.try_get_mut().expect("Forgot to insert Game?");
                        pointer_manager.run(game);
                    },
                );
                pointer_manager_activity.subscribe_domained_mut(
                    |pointer_manager, domain, msg: &mut WorldEvent| {
                        let game: &mut Game = domain.try_get_mut().expect("Forgot to insert Game?");
                        let event = msg.event();
                        let res = game.handle_quicksilver_event(&event, pointer_manager);
                        if let Err(e) = res {
                            nuts::publish(e);
                        }
                    },
                );
                // For debugging (Consider removing)
                pointer_manager_activity
                    .on_leave(|_| panic!("Pointer manager should not be deactived"));
                load_game_event_manager();

                Ok(())
            }
        }
    }
}
async fn load_image_from_variant(v: &AnimationVariantDef) -> PadlResult<Image> {
    Ok(match v {
        AnimationVariantDef::Animated(path) | AnimationVariantDef::Static(path) => {
            Image::load(*path).await?
        }
    })
}
fn start_loading_animations() -> Vec<impl std::future::Future<Output = PadlResult<AnimatedObject>>>
{
    ANIMATION_DEFS.iter().map(|a| load_animation(a)).collect()
}
async fn load_animation(def: &'static AnimatedObjectDef) -> PadlResult<AnimatedObject> {
    let cols = def.cols as u32;
    let rows = def.rows as u32;
    // TODO: Load in parallel
    let obj = AnimatedObject::walking(
        load_image_from_variant(&def.up).await?,
        load_image_from_variant(&def.left).await?,
        load_image_from_variant(&def.down).await?,
        cols,
        rows,
        load_image_from_variant(&def.standing).await?,
    );
    Ok(obj)
}

async fn start_loading_locale() -> PadlResult<TextDb> {
    let binary = paddle::load_file("locale/en.mo").await?;
    let tdb = TextDb::parse(binary.as_slice())
        .map_err(|_| ErrorMessage::technical("could not parse the catalog".to_owned()))?;
    Ok(tdb)
}

impl ScreenResolution {
    fn progress_bar_area_y(&self) -> f32 {
        match self {
            ScreenResolution::Low => 100.0,
            _ => self.pixels().1 * 0.618,
        }
    }
    fn progress_bar_area_h(&self) -> f32 {
        match self {
            ScreenResolution::Low => 100.0,
            ScreenResolution::Mid => 150.0,
            ScreenResolution::High => 200.0,
        }
    }
}

impl Frame for LoadingFrame {
    type State = Option<LoadScheduler>;
    type Error = PadlError;

    fn draw(
        &mut self,
        state: &mut Self::State,
        canvas: &mut WebGLCanvas,
        _timestamp: f64,
    ) -> Result<(), Self::Error> {
        if let Some(lm) = state.as_ref() {
            let progress = lm.progress();
            let msg = lm.waiting_for();
            self.draw_progress(canvas, progress, msg.unwrap_or("Done."))?;
        } else {
            self.draw_progress(canvas, 0.0, "Loading...")?;
        }
        Ok(())
    }
    fn update(&mut self, maybe_lm: &mut Self::State) -> Result<(), Self::Error> {
        if let Some(lm) = maybe_lm.as_mut() {
            loading_update_net(&mut self.net_chan, lm)?;
        }
        Ok(())
    }
}

impl GameLoadingData {
    pub fn try_from_loaded_data(loaded_data: &mut LoadedData) -> PadlResult<Self> {
        Ok(Self {
            player_info: *loaded_data.extract()?,
            worker_response: *loaded_data.extract()?,
            buildings_response: *loaded_data.extract()?,
            hobos_response: *loaded_data.extract()?,
            attacking_hobos: *loaded_data.extract()?,
            village_info: *loaded_data.extract()?,
            reports: *loaded_data.extract()?,
        })
    }
}
