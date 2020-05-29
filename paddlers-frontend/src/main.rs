#![recursion_limit = "512"]
#![feature(is_sorted, associated_type_bounds, vec_remove_item, const_fn)]
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

    // Initialize panes, enabling HTML access
    let resolution = crate::window::estimate_screen_size();
    let (w, h) = resolution.pixels();
    panes::init_ex(Some("game-root"), (0, 0), Some((w as u32, h as u32)))
        .expect("Panes initialization failed");

    // Set up loading state with interfaces to networking
    let (net_sender, net_receiver) = channel();
    let state = init::loading::LoadingState::new(resolution, net_receiver);
    let err_send = state.base.async_err.clone_sender();

    // Initialize networking
    net::init_net(net_sender, err_send);
    init::run(state);
}

pub use paddlers_shared_lib::shared_types::Timestamp;

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
