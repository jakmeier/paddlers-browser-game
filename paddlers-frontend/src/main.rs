#![recursion_limit="512"]
#![feature(is_sorted, associated_type_bounds, vec_remove_item)]
#[macro_use]
mod wasm_setup;
extern crate quicksilver;
#[macro_use]
extern crate stdweb;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod game;
mod gui;
mod net;
mod error;
mod prelude;

#[cfg(target_arch = "wasm32")]
use wasm_setup::setup_wasm;

use std::sync::mpsc::channel;

pub fn main() {
    #[cfg(target_arch = "wasm32")]
    setup_wasm();
    let (net_sender, net_receiver) = channel();
    game::run(1500.0, 600.0, net_receiver); // is cut to right ratio inside of game
    net::init_net(net_sender);
}


/// Micro second precision
pub type Timestamp = i64;
