use crate::prelude::*;
use crate::game::components::*;
use crate::net::NetMsg;
use quicksilver::prelude::*;
use specs::prelude::*;
use super::town::{Town, TOWN_RATIO};
use std::sync::mpsc::Receiver;
use super::specs_resources::insert_resources;

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
    let max_town_width = width - MENU_BOX_WIDTH;
    let (tw, th) = if max_town_width / height <= TOWN_RATIO {
        (max_town_width, max_town_width / TOWN_RATIO)
    } else {
        (TOWN_RATIO * height, height)
    };

    let ul = tw / super::town::X as f32;
    let menu_box_area = Rectangle::new((tw,0),(MENU_BOX_WIDTH, th));
    let main_area = Rectangle::new((0,0),(tw, th));
    quicksilver::lifecycle::run_with::<super::Game, _>(
        "Paddlers", 
        Vector::new(tw + MENU_BOX_WIDTH, th), 
        Settings::default(), 
        || Ok(
            super::Game::new().expect("Game initialization")
                .with_town(Town::new(ul)) // TODO: Think of a better way to handle unit lengths in general
                .with_unit_length(ul)
                .with_ui_division(main_area, menu_box_area)
                .with_network_chan(net_chan)
                .init_map()
            )
    );
}