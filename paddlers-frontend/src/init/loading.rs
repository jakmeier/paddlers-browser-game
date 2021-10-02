use crate::net::graphql::QuestsResponse;
use crate::net::graphql::{
    query_types::{
        AttacksResponse, BuildingsResponse, HobosQueryResponse, VolatileVillageInfoResponse,
    },
    ReportsResponse,
};
use crate::prelude::{PadlError, PadlErrorCode};
use crate::resolution::{SCREEN_H, SCREEN_W};
use crate::{game::net_receiver::loading_update_net, resolution::OUTER_MENU_AREA_W};
use nuts::LifecycleStatus;
use paddle::{
    DisplayArea, ErrorMessage, Frame, Image, LoadScheduler, LoadedData, LoadingDoneMsg,
    LoadingProgressMsg, NutsCheck, TextBoard,
};
use paddlers_shared_lib::specification_types::sprite_paths::SPRITE_PATHS;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::game::player_info::PlayerInfo;
use crate::game::Game;
use crate::gui::sprites::{
    animation::{AnimatedObject, AnimatedObjectDef, AnimationVariantDef},
    paths::ANIMATION_DEFS,
    Sprites,
};
use crate::gui::utils::*;
use crate::net::graphql::query_types::WorkerResponse;
use crate::net::NetMsg;
use crate::prelude::{PadlResult, TextDb};
use paddle::*;
use std::sync::mpsc::Receiver;

/// Published once the game has been initialized
pub struct PostInit;

/// State that is used while loading all data over the network.
/// It will automatically be removed when loading is done.
pub(crate) struct LoadingFrame {
    preload_float: FloatingText,
    net_chan: Receiver<NetMsg>,
    load_tracker: LoadSchedulerId,
    progress: f32,
    message: &'static str,
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
        let fh = paddle::register_frame(self, (), (0, 0));
        let aid = fh.activity();
        aid.subscribe_domained(|_loading_state, domain, msg: &LoadingProgressMsg| {
            domain.store(Some(msg.clone()));
        });
        aid.on_delete_domained(|loading_state, domain| {
            let loaded_data = std::mem::take(domain.get_mut::<LoadedData>());
            loading_state.finalize_loading(loaded_data).nuts_check();
        });
        aid.subscribe(move |_loading_state, _msg: &LoadingDoneMsg| {
            aid.set_status(LifecycleStatus::Deleted);
        });
        aid.subscribe(Self::on_loading_progress);
    }
    pub fn start(root_id: &str, net_chan: Receiver<NetMsg>) -> PadlResult<()> {
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
        Self::start_with_canvas(canvas, net_chan);
        Ok(())
    }
    pub fn start_with_canvas(canvas: HtmlCanvasElement, net_chan: Receiver<NetMsg>) {
        let texture_config =
            paddle::graphics::TextureConfig::default().with_bilinear_filtering_no_mipmaps();
        let config = paddle::PaddleConfig::default()
            .with_resolution((SCREEN_W, SCREEN_H))
            .with_canvas(canvas)
            .with_texture_config(texture_config)
            .with_text_board(Rectangle::new(
                (OUTER_MENU_AREA_W, 20),
                (SCREEN_W - (2 * OUTER_MENU_AREA_W), SCREEN_H - 40),
            ))
            .with_background_color(DARK_GREEN);
        paddle::init(config).expect("Failed creating window");

        let mut images = vec![];
        for src in &SPRITE_PATHS {
            let img = async move { Image::load(src).await };
            images.push(img);
        }
        let animations = start_loading_animations();
        let locale = start_loading_locale();

        let mut load_manager = LoadScheduler::new()
            .with_vec(images, "Drawing visuals for the game")
            .with_vec(animations, "Animating fellow Paddlers")
            .with(locale, "Writing localized texts")
            .with_manually_reported::<PlayerInfo>("Checking out player")
            .with_manually_reported::<WorkerResponse>("Summon working Paddlers")
            .with_manually_reported::<BuildingsResponse>("Construct buildings")
            .with_manually_reported::<HobosQueryResponse>("Summon non-working Paddlers")
            .with_manually_reported::<AttacksResponse>("Summon visitors")
            .with_manually_reported::<VolatileVillageInfoResponse>("Gather village news")
            .with_manually_reported::<ReportsResponse>("Gather letters from mailbox")
            .with_manually_reported::<QuestsResponse>("Listening to god's voice");

        // TODO: Ideally, this would directly call finalize_loading using the LoadedData received.
        // But right now, the LoadingFrame state is also required to take out the net_chan.
        // Thus an ugly indirection is used: 
        //   1) Store the LoadedData into the domain here
        //   2) Trigger a deletion on LoadingDoneMsg
        //   3) In the deletion handler take ownership of the LoadingFrame state and call finalize_loading
        // I'm sure there is a better way but probably requires some refactoring.
        load_manager
            .set_after_loading(|loaded_data| nuts::store_to_domain(&Domain::Frame, loaded_data));

        let load_tracker = load_manager.track_loading();

        let preload_float = FloatingText::try_default().expect("FloatingText");

        LoadingFrame {
            load_tracker,
            preload_float,
            net_chan,
            progress: 0.0,
            message: "Loading...",
        }
        .run_as_activity();
    }

    fn draw_progress(
        &mut self,
        window: &mut DisplayArea,
        progress: f32,
        msg: &str,
    ) -> PadlResult<()> {
        // TODO (optimization): Refactor to make this call event-based
        window.fit_display(20.0);

        let w = SCREEN_W as f32;
        let y = PROGRESS_BAR_AREA_Y;
        let ph = PROGRESS_BAR_AREA_H;
        let area = Rectangle::new((w * 0.1, y), (w * 0.8, ph));

        draw_progress_bar(window, &mut self.preload_float, area, progress, &msg)?;
        Ok(())
    }
    fn finalize_loading(self, mut loaded_data: LoadedData) -> PadlResult<()> {
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

        let net_chan = self.net_chan;
        let sprites = Sprites::new(images, animations);
        crate::gui::shapes::load_shapes();

        let game_data = GameLoadingData::try_from_loaded_data(&mut loaded_data)?;
        Game::register(move |display| {
            match Game::load_game(display, sprites, catalog, game_data, net_chan, loaded_data) {
                Err(e) => {
                    TextBoard::display_error_message(":(\nLoading game failed".to_owned())
                        .nuts_check(); // TODO: multi-lang errors
                    panic!("Fatal Error: Could not load game {:?}", e);
                }
                Ok(game) => {
                    // Note that at this point the game object has been created but is not stored to the Domain yet.
                    // This means that publishing messages with handlers that require the game state is not allowed, yet.
                    // That kind of initialization can be done in GameActivity::initialize_game
                    game
                }
            }
        });

        Ok(())
    }
    fn on_loading_progress(&mut self, msg: &LoadingProgressMsg) {
        self.progress = msg.progress;
        self.message = msg.description;
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
    let maybe_lang_id = crate::net::url::query_param("lang");
    let lang_id = maybe_lang_id.as_ref().map(String::as_str).unwrap_or("en");
    let binary = paddle::load_file(&format!("locale/{}.mo", lang_id)).await?;
    let tdb = TextDb::parse(binary.as_slice())
        .map_err(|_| ErrorMessage::technical("could not parse the catalog".to_owned()))?;
    Ok(tdb)
}

const PROGRESS_BAR_AREA_Y: f32 = 667.4;
const PROGRESS_BAR_AREA_H: f32 = 200.0;

impl Frame for LoadingFrame {
    type State = ();
    const WIDTH: u32 = SCREEN_W;
    const HEIGHT: u32 = SCREEN_H;

    fn draw(&mut self, _state: &mut Self::State, display: &mut DisplayArea, _timestamp: f64) {
        self.draw_progress(display, self.progress, self.message)
            .nuts_check();
    }
    fn update(&mut self, _state: &mut Self::State) {
        loading_update_net(&mut self.net_chan, self.load_tracker).nuts_check();
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
        })
    }
}
