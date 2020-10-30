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
use crate::init::loading::LoadingFrame;
use crate::prelude::*;
use paddle::quicksilver_compat::*;
use specs::prelude::*;
use specs_registration::{insert_global_resources, register_global_components};

pub(super) fn init_world(resolution: ScreenResolution, player_info: PlayerInfo) -> World {
    let mut world = World::new();

    // Components
    register_global_components(&mut world);

    // Resources
    insert_global_resources(&mut world, resolution, player_info);
    world
}
