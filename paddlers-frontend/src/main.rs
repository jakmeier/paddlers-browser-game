#![recursion_limit = "512"]
#![feature(is_sorted, associated_type_bounds, vec_remove_item)]
extern crate quicksilver;
#[macro_use]
extern crate stdweb;
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

#[cfg(target_arch = "wasm32")]
use init::wasm_setup::setup_wasm;

use std::sync::mpsc::channel;

pub fn main() {
    #[cfg(target_arch = "wasm32")]
    setup_wasm();
    let version = env!("CARGO_PKG_VERSION");
    println!("Paddlers {}", version);
    let (net_sender, net_receiver) = channel();
    net::init_net(net_sender);
    init::run(net_receiver);
}

/// Micro second precision
pub type Timestamp = i64;

use crate::prelude::PadlEvent;
use quicksilver::prelude::Window;
use view::FrameManager;
pub(crate) type Framer = FrameManager<
    gui::input::UiView,
    game::Game<'static, 'static>,
    Window,
    PadlEvent,
    prelude::PadlError,
    init::quicksilver_integration::Signal,
>;

#[inline]
pub fn seconds(t: Timestamp) -> i64 {
    t / 1_000_000
}
