//! This module collects setup code that is executed only once when loading.

/// Sets up some bindings to the browser to make things like println!() possible.
#[macro_use]
pub mod wasm_setup;

/// Handles the loading phase when all assets are downloaded
pub mod loading;

// doc comment inlined 
mod quicksilver_integration;

/// Boiler-plate code for initializing SPECS
pub mod specs_registration;

use crate::prelude::*;
use crate::game::town::Town;
use crate::net::NetMsg;
use quicksilver::prelude::*;
use specs::prelude::*;
use std::sync::mpsc::Receiver;
use specs_registration::{insert_resources, register_components};

pub (super) fn init_world(err_send: std::sync::mpsc::Sender<PadlError>) -> World {
    let mut world = World::new();

    // Components
    register_components(&mut world);

    // Resources
    insert_resources(&mut world, err_send);
    world
}
pub fn run(net_chan: Receiver<NetMsg>) {

    let resolution = crate::window::estimate_screen_size();

    let (w,h) = resolution.pixels();
    let (tw,th) = resolution.main_area();

    // Initialize panes
    panes::init_ex(Some("game-root"), (0,0), Some((tw as u32, th as u32))).expect("Panes initialization failed");
    
    // Load quicksilver canvas and loop
    let mut settings = Settings::default();
    settings.root_id = Some("game-root");
    quicksilver::lifecycle::run_with::<crate::game::Game, _>(
        "Paddlers", 
        Vector::new(w,h), 
        settings, 
        || Ok(
            crate::game::Game::new().expect("Game initialization")
                .with_town(Town::new(resolution))
                .with_resolution(resolution)
                .init_map()
                .init_views()
                .with_network_chan(net_chan)
            )
    );
}