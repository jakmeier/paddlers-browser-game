#![recursion_limit = "512"]
#![feature(
    is_sorted,
    associated_type_bounds,
    vec_remove_item,
    const_fn,
    const_fn_floating_point_arithmetic
)]
extern crate specs;
#[macro_use]
extern crate specs_derive;

#[macro_use]
mod init;

mod game;
mod gui;
mod i18n;
mod logging;
mod net;
mod prelude;
pub(crate) mod resolution;
mod view;
pub(crate) mod window;

use std::sync::mpsc::channel;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Paddlers {}", version);
    #[cfg(debug_assertions)]
    println!("Debug mode");

    crate::logging::init_error_handling();
    let resolution = crate::window::estimate_screen_size().expect("Reading window size failed");

    /* Now load the actual game */
    // Timing is key: network state should be registered in Nuts before rest of the game is loaded.
    // The problem is, error reporting is not yet ready...
    let (net_sender, net_receiver) = channel();
    net::init_net(net_sender);

    init::loading::LoadingFrame::start(resolution, "game-root", net_receiver)
        .expect("Failed loading.");

    // Now start loading data over the network.
    // To keep things simple, this done at the end, even if that means extra latency.
    crate::net::request_client_state();
}

pub use paddlers_shared_lib::shared_types::Timestamp;
