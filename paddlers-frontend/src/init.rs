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
use crate::init::quicksilver_integration::QuicksilverState;
use crate::logging::{text_to_user::TextBoard, AsyncErr, ErrorQueue};
use crate::net::game_master_api::RestApiState;
use crate::net::NetMsg;
use crate::prelude::*;
use quicksilver::prelude::*;
use specs::prelude::*;
use specs_registration::{insert_resources, register_components};
use std::sync::mpsc::Receiver;

pub(super) fn init_world(
    async_err: AsyncErr,
    resolution: ScreenResolution,
    player_info: PlayerInfo,
    rest: RestApiState,
    errq: ErrorQueue,
    tb: TextBoard,
) -> World {
    let mut world = World::new();

    // Components
    register_components(&mut world);

    // Resources
    insert_resources(
        &mut world,
        async_err,
        resolution,
        player_info,
        rest,
        errq,
        tb,
    );
    world
}
pub fn run(net_chan: Receiver<NetMsg>) {
    let resolution = crate::window::estimate_screen_size();

    let (w, h) = resolution.pixels();

    // Initialize panes
    panes::init_ex(Some("game-root"), (0, 0), Some((w as u32, h as u32)))
        .expect("Panes initialization failed");
    // Load quicksilver canvas and loop
    let mut settings = Settings::default();
    settings.root_id = Some("game-root");
    quicksilver::lifecycle::run_with::<QuicksilverState, _>(
        "Paddlers",
        Vector::new(w, h),
        settings,
        || Ok(QuicksilverState::load(resolution, net_chan)),
    );
}
