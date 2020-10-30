use crate::net::graphql::query_types::{
    AttacksResponse, BuildingsResponse, HobosQueryResponse, VolatileVillageInfoResponse,
};
use crate::{game::game_event_manager::load_game_event_manager, prelude::PadlError};
use crate::{game::leaderboard::doc, gui::input::UiView};
use crate::{game::net_receiver::loading_update_net, init::quicksilver_integration::Signal};
use futures::future::join_all;
use js_sys::JsString;
use nuts::LifecycleStatus;
use paddle::{
    graphics::Image, graphics::ImageLoader, ErrorMessage, Frame, JsError, LoadScheduler,
    LoadedData, LoadingDone, LoadingProgress, NutsCheck, TextBoard, UpdateWorld, Window,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
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
use std::{
    rc::Rc,
    sync::{mpsc::Receiver, Mutex},
};

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
}

impl LoadingFrame {
    fn run_as_activity(self) {
        let aid = paddle::frame_to_activity(self, &Domain::Frame);
        aid.subscribe_domained(|_loading_state, domain, msg: &LoadingProgress| {
            domain.store(Some(msg.clone()));
        });
        aid.on_delete_domained(|loading_state, domain| {
            let loaded_data = std::mem::take(domain.get_mut::<LoadedData>());
            loading_state.finalize(loaded_data);
        });
        aid.subscribe_domained(move |loading_state, domain, msg: &LoadingDone| {
            aid.set_status(LifecycleStatus::Deleted);
        });
        let draw_handle = super::quicksilver_integration::start_drawing();
        let update_handle = super::quicksilver_integration::start_updating();
        nuts::store_to_domain(&Domain::Frame, (draw_handle, update_handle));
    }
    pub fn start(
        resolution: ScreenResolution,
        root_id: &str,
        net_chan: Receiver<NetMsg>,
    ) -> PadlResult<()> {
        let document = doc()?;
        let root = document.get_element_by_id(root_id);
        let canvas = document
            .create_element("canvas")
            .map_err(|_| "canvas creation failed")?
            .dyn_into()
            .unwrap();
        Self::start_with_canvas(resolution, canvas, net_chan);
        Ok(())
    }
    pub fn start_with_canvas(
        resolution: ScreenResolution,
        canvas: HtmlCanvasElement,
        net_chan: Receiver<NetMsg>,
    ) {
        crate::net::request_client_state();
        let canvas = Window::new(canvas).expect("Failed creating window");
        ImageLoader::register(canvas.clone_webgl());
        nuts::store_to_domain(&Domain::Frame, canvas);
        let mut images = vec![];
        for src in &SPRITE_PATHS {
            let img = async move { Image::load(src).await };
            images.push(img);
        }
        let locale = start_loading_locale();

        let load_manager = LoadScheduler::new()
            .with_vec(images, "Drawing visuals for the game")
            .with(locale, "Writing localized texts")
            .with_manually_reported::<NetMsg>("Collecting news in Paddland")
            .with_manually_reported::<PlayerInfo>("Downloading player data")
            .with_manually_reported::<WorkerResponse>("Summon working Paddlers")
            .with_manually_reported::<BuildingsResponse>("Construct buildings")
            .with_manually_reported::<HobosQueryResponse>("Summon non-working Paddlers")
            .with_manually_reported::<AttacksResponse>("Summon visitors")
            .with_manually_reported::<VolatileVillageInfoResponse>("Gather village news");

        load_manager.attach_to_domain();

        let preload_float = FloatingText::try_default().expect("FloatingText");

        LoadingFrame {
            resolution,
            preload_float,
            net_chan,
        }
        .run_as_activity();
    }
    // pub fn progress(&mut self) -> (f32, &'static str) {
    //     let images_loaded = self
    //         .images
    //         .iter_mut()
    //         .map(Self::asset_loaded)
    //         .filter(|b| *b)
    //         .count();
    //     self.progress.report_progress::<Image>(images_loaded);
    //     let locale_loaded = if Self::asset_loaded(&mut self.locale) {
    //         1
    //     } else {
    //         0
    //     };
    //     self.progress.report_progress::<TextDb>(locale_loaded);
    //     let p = self.progress.progress();
    //     // This could be handled nicer by a separate loader object but I kept it simple for now
    //     let msg = self.progress.waiting_for();
    //     (p, msg)
    // }
    // pub fn draw_loading(&mut self, window: &mut Window) -> PadlResult<()> {
    //     let (progress, msg) = self.progress();

    //     self.draw_progress(window, progress, msg)?;
    //     Ok(())
    // }
    fn draw_progress(&mut self, window: &mut Window, progress: f32, msg: &str) -> PadlResult<()> {
        window.clear(DARK_GREEN);
        let r = self.resolution;
        let w = r.pixels().0;
        let y = r.progress_bar_area_y();
        let ph = r.progress_bar_area_h();
        let area = Rectangle::new((w * 0.1, y), (w * 0.8, ph));

        draw_progress_bar(window, &mut self.preload_float, area, progress, &msg)
    }
    pub fn queue_error(&mut self, res: PadlResult<()>) {
        if let Err(e) = res {
            nuts::publish(e);
        }
    }
    fn finalize(self, mut loaded_data: LoadedData) -> PadlResult<()> {
        let images = loaded_data.extract_vec()?;
        let catalog = loaded_data.extract()?;
        let (resolution, net_chan) = (self.resolution, self.net_chan);
        let sprites = Sprites::new(images);

        let game_data = GameLoadingData::from_boxes(
            loaded_data.extract()?,
            loaded_data.extract()?,
            loaded_data.extract()?,
            loaded_data.extract()?,
            loaded_data.extract()?,
            loaded_data.extract()?,
        );

        let viewer_data: Vec<NetMsg> = *loaded_data.extract()?;

        match Game::load_game(sprites, *catalog, resolution, game_data, net_chan) {
            Err(e) => {
                TextBoard::display_error_message(":(\nLoading game failed".to_owned()).nuts_check(); // TODO: multi-lang errors
                panic!("Fatal Error: Could not load game {:?}", e);
            }
            Ok(mut game) => {
                let pointer_manager =
                    crate::gui::input::pointer::PointerManager::init(&mut game.world);
                nuts::store_to_domain(&Domain::Frame, game);
                let view = UiView::Town;
                let viewer = super::frame_loading::load_viewer(view, resolution);
                for evt in viewer_data {
                    paddle::share(evt);
                }
                paddle::share_foreground(Signal::ResourcesUpdated);

                let viewer_activity = nuts::new_domained_activity(viewer, &Domain::Frame);
                viewer_activity.subscribe_domained(|viewer, domain, _: &UpdateWorld| {
                    let game: &mut Game =
                        domain.try_get_mut().expect("Forgot to insert Game?");
                    // FIXME; really need to be set every frame?
                    let view: UiView = *game.world.fetch();
                    viewer.set_view(view);
                });

                let pointer_manager_activity =
                    nuts::new_domained_activity(pointer_manager, &Domain::Frame);
                pointer_manager_activity.subscribe_domained(
                    |pointer_manager, domain, _: &UpdateWorld| {
                        let game: &mut Game =
                            domain.try_get_mut().expect("Forgot to insert Game?");
                        pointer_manager.run(game);
                    },
                );
                pointer_manager_activity.subscribe_domained_mut(
                    |pointer_manager, domain, msg: &mut WorldEvent| {
                        let game: &mut Game =
                            domain.try_get_mut().expect("Forgot to insert Game?");
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
    // TODO
    // async fn load_image_from_variant(&self, v: &AnimationVariantDef) -> PadlResult<Image> {
    //     match v {
    //         AnimationVariantDef::Animated(path) | AnimationVariantDef::Static(path) => {
    //             let canvas = self.base.canvas.lock().unwrap();
    //             Ok(canvas.load_image(*path).await?)
    //         }
    //     }
    // }
    // TODO
    // pub async fn start_loading_animations(&self, images: &Vec<Image>){
    //     ANIMATION_DEFS
    //         .iter()
    //         .for_each(|a| { self.load_animation(a, images).await; });
    // }
    // // Potential bug? https://github.com/rust-lang/rust/issues/63033
    // async fn load_animation(
    //     &self,
    //     def: &'static AnimatedObjectDef,
    //     images: &Vec<Image>,
    // ) -> PadlResult<(AnimatedObject, Image)> {
    //     let futures = join_all(vec![
    //         self.load_image_from_variant(&def.up),
    //         self.load_image_from_variant(&def.left),
    //         self.load_image_from_variant(&def.down),
    //         self.load_image_from_variant(&def.standing),
    //     ]);
    //     let cols = def.cols as u32;
    //     let rows = def.rows as u32;

    //     let mut iter = futures.await.into_iter();
    //     let obj = AnimatedObject::walking(
    //         iter.next().unwrap()?,
    //         iter.next().unwrap()?,
    //         iter.next().unwrap()?,
    //         cols,
    //         rows,
    //         iter.next().unwrap()?,
    //     );
    //     Ok((obj, images[def.alternative.index_in_vector()].clone()))
    // }
}

async fn start_loading_locale() -> PadlResult<TextDb> {
    let binary = paddle::load_file("locale/en.mo").await?;
    let tdb = TextDb::parse(binary.as_slice())
        .map_err(|_| ErrorMessage::technical("could not parse the catalog".to_owned()))?;
    Ok(tdb)
}

// impl QuicksilverState {
//     pub(crate) fn try_finalize(&mut self) {
//         match self {
//             Self::Loading(state) => {
//                 if state.progress.done() {
//                     let err = state.preload_float.hide();
//                     state.queue_error(err.map_err(|e| e.into()));
//                     self.finalize();
//                     crate::net::activate_net();
//                 }
//             }
//             _ => println!("Attempted second finalization"),
//         }
//     }
//     fn finalize(&mut self) {
//         let moved_state = std::mem::replace(self, QuicksilverState::Empty);
//         match moved_state {
//             Self::Loading(state) => {
//                 Game::register_in_nuts();
//                 state.finalize();
//                 *self = QuicksilverState::Ready;
//             }
//             _ => unreachable!(),
//         }
//     }

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
    type Graphics = Window;

    fn draw(
        &mut self,
        state: &mut Self::State,
        canvas: &mut Self::Graphics,
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
    pub fn from_boxes(
        player_info: Box<PlayerInfo>,
        worker_response: Box<WorkerResponse>,
        buildings_response: Box<BuildingsResponse>,
        hobos_response: Box<HobosQueryResponse>,
        attacking_hobos: Box<AttacksResponse>,
        village_info: Box<VolatileVillageInfoResponse>,
    ) -> Self {
        Self {
            player_info: *player_info,
            worker_response: *worker_response,
            buildings_response: *buildings_response,
            hobos_response: *hobos_response,
            attacking_hobos: *attacking_hobos,
            village_info: *village_info,
        }
    }
}
