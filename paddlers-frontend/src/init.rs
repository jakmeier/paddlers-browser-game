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
use crate::game::town::{Town, TOWN_RATIO};
use crate::net::NetMsg;
use quicksilver::prelude::*;
use specs::prelude::*;
use std::sync::mpsc::Receiver;
use specs_registration::{insert_resources, register_components};

const MENU_BOX_WIDTH: f32 = 300.0;

pub (super) fn init_world(err_send: std::sync::mpsc::Sender<PadlError>) -> World {
    let mut world = World::new();

    // Components
    register_components(&mut world);

    // Resources
    insert_resources(&mut world, err_send);
    world
}
pub fn run(width: f32, height: f32, net_chan: Receiver<NetMsg>) {
    // Cut window ratio to something that does not distort the geometry
    let max_town_width = width - MENU_BOX_WIDTH;
    let (tw, th) = if max_town_width / height <= TOWN_RATIO {
        (max_town_width, max_town_width / TOWN_RATIO)
    } else {
        (TOWN_RATIO * height, height)
    };

    let ul = tw / crate::game::town::X as f32;
    let menu_box_area = Rectangle::new((tw,0),(MENU_BOX_WIDTH, th));
    let main_area = Rectangle::new((0,0),(tw, th));

    // Initialize panes
    panes::init_ex("game-root", 0, 0, tw as u32, th as u32).expect("Panes initialization failed");
    
    // Load quicksilver canvas and loop
    quicksilver::lifecycle::run_with::<crate::game::Game, _>(
        "Paddlers", 
        Vector::new(tw + MENU_BOX_WIDTH, th), 
        Settings::default(), 
        || Ok(
            crate::game::Game::new().expect("Game initialization")
                .with_town(Town::new(ul)) // TODO: Think of a better way to handle unit lengths in general
                .with_unit_length(ul)
                .with_ui_division(main_area, menu_box_area)
                .with_network_chan(net_chan)
                .init_map()
                .init_views()
            )
    );
}