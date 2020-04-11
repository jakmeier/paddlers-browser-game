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
use crate::logging::error::PadlError;
use crate::logging::text_to_user::TextBoard;
use crate::logging::ErrorQueue;
use crate::net::game_master_api::RestApiState;
use crate::net::NetMsg;
use crate::prelude::{Catalog, PadlResult, ScreenResolution};
use crate::view::FloatingText;
use quicksilver::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};

/// State which must always be present to enable basic tasks
/// like networking and error handling.
pub struct BaseState {
    pub err_recv: Receiver<PadlError>,
    pub err_send: Sender<PadlError>,
    pub net_chan: Receiver<NetMsg>,
    pub rest: RestApiState,
    pub errq: ErrorQueue,
    pub tb: TextBoard,
}

/// State that is used while loading all data over the network
pub struct LoadingState {
    pub player_info: Option<PlayerInfo>,
    pub base: BaseState,
    images: Vec<Asset<Image>>,
    locale: Asset<Catalog>,
    resolution: ScreenResolution,
    preload_float: FloatingText,
}

impl LoadingState {
    pub fn new(resolution: ScreenResolution, net_chan: Receiver<NetMsg>) -> Self {
        let images = start_loading_sprites();
        let locale = start_loading_locale();
        crate::net::request_player_update();
        let preload_float = FloatingText::try_default().expect("FloatingText");
        let (err_send, err_recv) = channel();
        let err_send_clone = err_send.clone();
        let rest = RestApiState::new(err_send_clone);
        let base = BaseState {
            err_recv,
            err_send,
            net_chan,
            rest,
            errq: ErrorQueue::default(),
            tb: TextBoard::default(),
        };
        LoadingState {
            base,
            player_info: None,
            images,
            locale,
            resolution,
            preload_float,
        }
    }
    pub fn progress(&mut self) -> f32 {
        let total = self.images.len() as f32 + 2.0;
        let images_loaded = self
            .images
            .iter_mut()
            .map(Self::asset_loaded)
            .filter(|b| *b)
            .count();
        let locale_loaded = if Self::asset_loaded(&mut self.locale) {
            1
        } else {
            0
        };
        let player_info_loaded = if self.player_info.is_some() { 1 } else { 0 };
        (images_loaded + locale_loaded + player_info_loaded) as f32 / total
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
        let progress = self.progress();
        self.draw_progress(window, progress)?;
        Ok(())
    }
    fn draw_progress(&mut self, window: &mut Window, progress: f32) -> PadlResult<()> {
        window.clear(DARK_GREEN)?;
        let r = self.resolution;
        let w = r.pixels().0;
        let y = r.progress_bar_area_y();
        let ph = r.progress_bar_area_h();
        let area = Rectangle::new((w * 0.1, y), (w * 0.8, ph));

        // This could be handled nicer by a separate loader object but I kept it simple for now
        let msg = if !self.player_info.is_some() {
            "Downloading player data"
        } else if !Self::asset_loaded(&mut self.locale) {
            "Downloading localized texts"
        } else {
            "Downloading images"
        };
        draw_progress_bar(window, &mut self.preload_float, area, progress, &msg)
    }
    pub fn queue_error(&mut self, res: PadlResult<()>) {
        if let Err(e) = res {
            self.base.errq.push(e);
        }
    }
    fn finalize(self) -> GameState {
        let (images, catalog, resolution, player_info, base) = {
            (
                self.images.into_iter().map(LoadingState::extract_asset).collect(),
                LoadingState::extract_asset(self.locale),
                self.resolution,
                self.player_info.expect("Finalized without player info"),
                self.base,
            )
        };
        let sprites = Sprites::new(images);
        match Game::load_game(sprites, catalog, resolution, player_info, base) {
            Err(e) => {
                panic!("Fatal Error: Could not load game {:?}", e);
            }
            Ok((mut game, ep)) => {
                let pm = crate::gui::input::pointer::PointerManager::init(
                    &mut game.world,
                    ep.clone(),
                );
                let viewer = super::frame_loading::load_viewer(&mut game, ep);
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
                if state.progress() >= 1.0 {
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
            _ => unreachable!()
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

fn start_loading_locale() -> Asset<Catalog> {
    Asset::new(
        // TODO
        quicksilver::load_file("locale/en.mo")
            .map(|data| Catalog::parse(data.as_slice()).expect("could not parse the catalog")),
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
