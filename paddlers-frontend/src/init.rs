//! This module collects setup code that is executed only once when loading.

/// Sets up some bindings to the browser to make things like println!() possible.
#[macro_use]
pub mod wasm_setup;

/// Handles the loading phase when all assets are downloaded
pub mod loading;

// doc comment inlined
pub(crate) mod quicksilver_integration;

mod frame_loading;
/// Boiler-plate code for initializing SPECS
pub mod specs_registration;

use crate::game::player_info::PlayerInfo;
use crate::init::loading::LoadingState;
use crate::init::quicksilver_integration::QuicksilverState;
use crate::logging::{text_to_user::TextBoard, AsyncErr, ErrorQueue};
use crate::prelude::*;
use quicksilver::prelude::*;
use specs::prelude::*;
use specs_registration::{insert_global_resources, register_global_components};

pub(super) fn init_world(
    async_err: AsyncErr,
    resolution: ScreenResolution,
    player_info: PlayerInfo,
    errq: ErrorQueue,
    tb: TextBoard,
) -> World {
    let mut world = World::new();

    // Components
    register_global_components(&mut world);

    // Resources
    insert_global_resources(&mut world, async_err, resolution, player_info, errq, tb);
    world
}
pub(crate) fn run(state: LoadingState) {
    let (w, h) = state.resolution.pixels();

    // Load quicksilver canvas and loop
    let mut settings = Settings::default();
    settings.root_id = Some("game-root");
    quicksilver::lifecycle::run_with::<QuicksilverState, _>(
        "Paddlers",
        Vector::new(w, h),
        settings,
        || Ok(QuicksilverState::load(state)),
    );
}
