//! This module collects setup code that is executed only once when loading.

/// Sets up some bindings to the browser to make things like println!() possible.
#[macro_use]
pub mod wasm_setup;

/// Handles the loading phase when all assets are downloaded
pub mod loading;

// doc comment inlined 
pub (crate) mod quicksilver_integration;

/// Boiler-plate code for initializing SPECS
pub mod specs_registration;

use crate::prelude::*;
use crate::net::NetMsg;
use crate::init::quicksilver_integration::QuicksilverState;
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

    // Initialize panes
    panes::init_ex(Some("game-root"), (0,0), Some((w as u32, h as u32))).expect("Panes initialization failed");
    
    // Load quicksilver canvas and loop
    let mut settings = Settings::default();
    settings.root_id = Some("game-root");
    quicksilver::lifecycle::run_with::<QuicksilverState, _>(
        "Paddlers", 
        Vector::new(w,h), 
        settings, 
        || Ok(QuicksilverState::load(resolution, net_chan))
    );
}