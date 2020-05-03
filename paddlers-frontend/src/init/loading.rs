mod progress_manager;
use crate::init::quicksilver_integration::{PadlEvent, Signal};
use crate::net::graphql::query_types::{
    AttacksResponse, BuildingsResponse, HobosQueryResponse, VolatileVillageInfoResponse,
};
use progress_manager::*;

use crate::game::player_info::PlayerInfo;
use crate::game::Game;
use crate::gui::sprites::{
    animation::{AnimatedObject, AnimatedObjectDef, AnimationVariantDef},
    paths::{ANIMATION_DEFS, SPRITE_PATHS},
    Sprites,
};
use crate::gui::utils::*;
use crate::init::quicksilver_integration::GameState;
use crate::init::quicksilver_integration::QuicksilverState;
use crate::logging::{error::PadlError, text_to_user::TextBoard, AsyncErr, ErrorQueue};
use crate::net::graphql::query_types::WorkerResponse;
use crate::net::NetMsg;
use crate::prelude::{PadlResult, ScreenResolution, TextDb};
use crate::view::FloatingText;
use quicksilver::prelude::*;
use std::sync::mpsc::{channel, Receiver};

/// State which must always be present to enable basic tasks
/// like networking and error handling.
pub struct BaseState {
    pub err_recv: Receiver<PadlError>,
    pub async_err: AsyncErr,
    pub net_chan: Receiver<NetMsg>,
    pub errq: ErrorQueue,
    pub tb: TextBoard,
}

/// State that is used while loading all data over the network
pub(crate) struct LoadingState {
    pub progress: ProgressManager,
    pub game_data: GameLoadingData,
    pub viewer_data: Vec<PadlEvent>,
    pub base: BaseState,
    pub resolution: ScreenResolution,
    images: Vec<Asset<Image>>,
    locale: Asset<TextDb>,
    preload_float: FloatingText,
}

#[derive(Default)]
pub struct GameLoadingData {
    pub player_info: Option<PlayerInfo>,
    pub worker_response: Option<WorkerResponse>,
    pub buildings_response: Option<BuildingsResponse>,
    pub hobos_response: Option<HobosQueryResponse>,
    pub attacking_hobos: Option<AttacksResponse>,
    pub village_info: Option<VolatileVillageInfoResponse>,
}

impl LoadingState {
    pub fn new(resolution: ScreenResolution, net_chan: Receiver<NetMsg>) -> Self {
        let images = start_loading_sprites();
        let locale = start_loading_locale();
        crate::net::request_client_state();
        let preload_float = FloatingText::try_default().expect("FloatingText");
        let (err_send, err_recv) = channel();
        let async_err = AsyncErr::new(err_send);
        let base = BaseState {
            err_recv,
            async_err,
            net_chan,
            errq: ErrorQueue::default(),
            tb: TextBoard::default(),
        };

        let game_data = GameLoadingData::default();
        // For leaderboard network event
        let viewer_data = vec![];
        let progress = ProgressManager::new()
            .with::<PadlEvent>(1, "Downloading news in Paddland")
            .with_loadable(&game_data.player_info, "Downloading player data")
            .with_loadable(&game_data.worker_response, "Downloading working Paddlers")
            .with_loadable(&game_data.buildings_response, "Downloading buildings")
            .with_loadable(
                &game_data.hobos_response,
                "Downloading non-working Paddlers",
            )
            .with_loadable(&game_data.attacking_hobos, "Downloading visitors")
            .with_loadable(&game_data.village_info, "Downloading village news")
            .with::<TextDb>(1, "Downloading localized texts")
            .with::<Image>(images.len(), "Downloading images");
        LoadingState {
            base,
            game_data,
            images,
            locale,
            resolution,
            preload_float,
            progress,
            viewer_data,
        }
    }
    pub fn progress(&mut self) -> (f32, &'static str) {
        let images_loaded = self
            .images
            .iter_mut()
            .map(Self::asset_loaded)
            .filter(|b| *b)
            .count();
        self.progress.report_progress::<Image>(images_loaded);
        let locale_loaded = if Self::asset_loaded(&mut self.locale) {
            1
        } else {
            0
        };
        self.progress.report_progress::<TextDb>(locale_loaded);
        let p = self.progress.progress();
        // This could be handled nicer by a separate loader object but I kept it simple for now
        let msg = self.progress.waiting_for();
        (p, msg)
    }
    pub fn asset_loaded<T>(asset: &mut Asset<T>) -> bool {
        let mut helper = false;
        asset
            .execute(|_| {
                helper = true;
                Ok(())
            })
            .unwrap();
        helper
    }
    pub fn extract_asset<T: Clone>(mut asset: Asset<T>) -> T {
        let mut helper = None;
        asset
            .execute(|img| {
                helper = Some(img.clone());
                Ok(())
            })
            .unwrap();
        helper.unwrap()
    }
    pub fn draw_loading(&mut self, window: &mut Window) -> PadlResult<()> {
        let (progress, msg) = self.progress();

        self.draw_progress(window, progress, msg)?;
        Ok(())
    }
    fn draw_progress(&mut self, window: &mut Window, progress: f32, msg: &str) -> PadlResult<()> {
        window.clear(DARK_GREEN)?;
        let r = self.resolution;
        let w = r.pixels().0;
        let y = r.progress_bar_area_y();
        let ph = r.progress_bar_area_h();
        let area = Rectangle::new((w * 0.1, y), (w * 0.8, ph));

        draw_progress_bar(window, &mut self.preload_float, area, progress, &msg)
    }
    pub fn queue_error(&mut self, res: PadlResult<()>) {
        if let Err(e) = res {
            self.base.errq.push(e);
        }
    }
    fn finalize(self) -> GameState {
        let (images, catalog, resolution, game_data, base) = {
            (
                self.images
                    .into_iter()
                    .map(LoadingState::extract_asset)
                    .collect(),
                LoadingState::extract_asset(self.locale),
                self.resolution,
                self.game_data,
                self.base,
            )
        };
        let sprites = Sprites::new(images);
        match Game::load_game(sprites, catalog, resolution, game_data, base) {
            Err(e) => {
                let mut tb = TextBoard::default();
                #[allow(unused_must_use)]
                {
                    tb.display_error_message(":(\nLoading game failed".to_owned()); // TODO: multi-lang errors
                    tb.draw(&Rectangle::new_sized(resolution.main_area()));
                }
                panic!("Fatal Error: Could not load game {:?}", e);
            }
            Ok(mut game) => {
                let pm = crate::gui::input::pointer::PointerManager::init(&mut game.world);
                let ep = game.event_pool.clone();
                let mut viewer = super::frame_loading::load_viewer(&mut game, ep);
                for evt in self.viewer_data {
                    let e = viewer.global_event(&mut game, &evt);
                    game.check(e);
                }
                let e = viewer.event(&mut game, &PadlEvent::Signal(Signal::ResourcesUpdated));
                game.check(e);
                GameState {
                    game,
                    viewer,
                    pointer_manager: pm,
                }
            }
        }
    }
}
impl QuicksilverState {
    pub(crate) fn try_finalize(&mut self) {
        match self {
            Self::Loading(state) => {
                if state.progress.done() {
                    let err = state.preload_float.hide();
                    state.queue_error(err.map_err(|e| e.into()));
                    self.finalize();
                    crate::net::activate_net();
                }
            }
            _ => println!("Attempted second finalization"),
        }
    }
    fn finalize(&mut self) {
        let moved_state = std::mem::replace(self, QuicksilverState::Empty);
        match moved_state {
            Self::Loading(state) => {
                *self = QuicksilverState::Ready(state.finalize());
            }
            _ => unreachable!(),
        }
    }
}

fn start_loading_sprites() -> Vec<Asset<Image>> {
    let images: Vec<Asset<Image>> = SPRITE_PATHS.iter().map(load_image).collect();
    images
}

pub fn start_loading_animations(images: &Vec<Image>) -> Vec<(Asset<AnimatedObject>, Image)> {
    let animations = ANIMATION_DEFS
        .iter()
        .map(|a| load_animation(a, images))
        .collect();
    animations
}

fn start_loading_locale() -> Asset<TextDb> {
    Asset::new(
        // TODO
        quicksilver::load_file("locale/en.mo")
            .map(|data| TextDb::parse(data.as_slice()).expect("could not parse the catalog")),
    )
}

fn load_image(path: &&'static str) -> Asset<Image> {
    Asset::new(Image::load(*path))
}
fn load_image_from_variant(v: &AnimationVariantDef) -> impl Future<Item = Image, Error = Error> {
    match v {
        AnimationVariantDef::Animated(path) | AnimationVariantDef::Static(path) => {
            Image::load(*path)
        }
    }
}
fn load_animation(
    def: &'static AnimatedObjectDef,
    images: &Vec<Image>,
) -> (Asset<AnimatedObject>, Image) {
    let futures = join_all(vec![
        load_image_from_variant(&def.up),
        load_image_from_variant(&def.left),
        load_image_from_variant(&def.down),
        load_image_from_variant(&def.standing),
    ]);
    let cols = def.cols as u32;
    let rows = def.rows as u32;
    let obj = futures.map(move |res| {
        let mut iter = res.into_iter();
        AnimatedObject::walking(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            cols,
            rows,
            iter.next().unwrap(),
        )
    });
    (
        Asset::new(obj),
        images[def.alternative.index_in_vector()].clone(),
    )
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
