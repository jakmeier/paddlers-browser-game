/// This module keeps the logic to read input and, in most cases,
/// redirect it to suitable modules to handle the input

use quicksilver::prelude::*;
use quicksilver::geom::Vector;
use specs::prelude::*;
use paddlers_shared_lib::prelude::*;
use crate::prelude::*;
use crate::net::state::current_village;
use crate::game::fight::*;
use crate::game::movement::Position;
use crate::gui::input::pointer::PointerManager;
use crate::gui::ui_state::Now;

pub mod pointer;
pub mod drag;
pub mod left_click;
pub mod right_click;
pub mod hover;

pub use self::{
    left_click::*,
    right_click::*,
    hover::*,
};
use crate::gui::ui_state::UiState;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub Option<MouseButton>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiView {
    Attacks,
    Leaderboard,
    Map,
    Town,
    Dialog,
}

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone)]
pub enum Grabbable {
    NewBuilding(BuildingType),
    Ability(AbilityType),
}

impl crate::game::Game<'_, '_> {
    pub fn handle_event(&mut self, event: &Event, window: &mut Window, pointer_manager: &mut PointerManager) -> Result<()> {
        // println!("Event: {:?}", event);
        // {
        //     let mut t = self.world.write_resource::<TextBoard>();
        //     t.display_debug_message(format!("{:?}", event));
        // }
        match event {
            Event::MouseMoved(pos) => {
                pointer_manager.move_pointer(&mut self.world, &pos);
            },
            Event::MouseButton(button, state)
            => {
                let now = self.world.read_resource::<Now>().0;
                let pos = &window.mouse().pos();
                pointer_manager.button_event(now, pos, *button, *state);  
            }
            Event::Key(key, state) 
                if *key == Key::Escape && *state == ButtonState::Pressed =>
                {
                    let mut ui_state = self.world.write_resource::<UiState>();
                    if (*ui_state).grabbed_item.is_some(){
                        (*ui_state).grabbed_item = None;
                    } else {
                        (*ui_state).selected_entity = None;
                    }
                },
            Event::Key(key, state) 
                if *key == Key::Delete && *state == ButtonState::Pressed =>
                {
                    let mut ui_state = self.world.write_resource::<UiState>();
                    let view = (*ui_state).current_view;
                    match view {
                        UiView::Town => {
                            if let Some(e) = ui_state.selected_entity {
                                (*ui_state).selected_entity = None;
                                
                                std::mem::drop(ui_state);

                                let pos_store = self.world.read_storage::<Position>();
                                let pos = pos_store.get(e).unwrap();
                                let tile_index = self.town().tile(pos.area.center());
                                std::mem::drop(pos_store);

                                let r = self.rest().http_delete_building(tile_index, current_village());
                                self.check(r);

                                // Account for changes in aura total
                                let aura_store = self.world.read_storage::<Aura>();
                                let aura = aura_store.get(e).map(|a| a.effect);
                                let range_store = self.world.read_storage::<Range>();
                                let range = range_store.get(e).map(|r| r.range);
                                std::mem::drop(aura_store);
                                std::mem::drop(range_store);
                                if let Some(aura) = aura {
                                    if let Some(range) = range {
                                        if range > self.town().distance_to_lane(tile_index) {
                                            self.town_mut().total_ambience -= aura;
                                        }
                                    }
                                }

                                self.town_mut().remove_building(tile_index);
                                self.world.delete_entity(e)
                                    .unwrap_or_else(
                                        |_|
                                        self.check(
                                            PadlErrorCode::DevMsg("Tried to delete wrong Generation").dev()
                                        ).unwrap()
                                    );
                            }
                        },
                        _ => {},
                    }
                },
            Event::Key(key, state) 
                if *key == Key::Tab && *state == ButtonState::Pressed =>
                {
                    self.toggle_view();
                },
            _evt => {
                // println!("Event: {:#?}", _evt)
            }
        };
        #[cfg(feature="dev_view")]
        self.dev_view_event(event);
        self.world.maintain();
        Ok(())
    }
}