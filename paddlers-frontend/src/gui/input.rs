use crate::game::fight::*;
use crate::game::movement::Position;
use crate::gui::input::pointer::PointerManager;
use crate::gui::ui_state::Now;
use crate::net::game_master_api::RestApiState;
use crate::net::state::current_village;
use crate::prelude::*;
/// This module keeps the logic to read input and, in most cases,
/// redirect it to suitable modules to handle the input
use paddle::quicksilver_compat::*;
use paddle::{quicksilver_compat::Vector, Window};
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

pub mod drag;
pub mod hover;
pub mod left_click;
pub mod pointer;
pub use self::{hover::*, left_click::*};
use crate::gui::ui_state::UiState;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub Option<MouseButton>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiView {
    Visitors(VisitorViewTab),
    Leaderboard,
    Map,
    Town,
    Dialogue,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VisitorViewTab {
    IncomingAttacks,
    Letters,
}

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone, Debug)]
pub enum Grabbable {
    NewBuilding(BuildingType),
    Ability(AbilityType),
}

impl crate::game::Game<'_, '_> {
    pub fn handle_quicksilver_event(
        &mut self,
        event: &Event,
        window: &mut Window,
        pointer_manager: &mut PointerManager,
    ) -> PadlResult<()> {
        match event {
            Event::MouseMoved(pos) => {
                pointer_manager.move_pointer(&mut self.world, &pos);
                self.prepare_town_resources();
                pointer_manager.move_pointer(self.town_world_mut(), &pos);
            }
            Event::MouseButton(button, state) => {
                let now = self.world.read_resource::<Now>().0;
                let pos = &self.mouse.pos();
                pointer_manager.button_event(now, pos, *button, *state);
            }
            Event::Key(key, state) if *key == Key::Escape && *state == ButtonState::Pressed => {
                let mut ui_state = self.world.write_resource::<UiState>();
                if ui_state.take_grabbed_item().is_none() {
                    ui_state.selected_entity = None;
                }
                let mut ui_state = self.town_world().write_resource::<UiState>();
                if ui_state.take_grabbed_item().is_none() {
                    ui_state.selected_entity = None;
                }
            }
            Event::Key(key, state) if *key == Key::Delete && *state == ButtonState::Pressed => {
                let view = *self.world.fetch::<UiView>();
                let town_world = self.town_world();
                let mut ui_state = town_world.write_resource::<UiState>();
                match view {
                    UiView::Town => {
                        if let Some(e) = ui_state.selected_entity {
                            (*ui_state).selected_entity = None;

                            std::mem::drop(ui_state);

                            let pos_store = town_world.read_storage::<Position>();
                            let resolution = town_world.read_resource::<ScreenResolution>();
                            let pos = pos_store.get(e).unwrap();
                            let tile_index = resolution.tile(pos.area.center());
                            std::mem::drop(pos_store);
                            std::mem::drop(resolution);

                            let r = RestApiState::get()
                                .http_delete_building(tile_index, current_village());
                            self.check(r);

                            // Account for changes in aura total
                            let aura_store = town_world.read_storage::<Aura>();
                            let aura = aura_store.get(e).map(|a| a.effect);
                            let range_store = town_world.read_storage::<Range>();
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
                            self.town_world_mut().delete_entity(e).unwrap_or_else(|_| {
                                self.check(
                                    PadlErrorCode::DevMsg("Tried to delete wrong Generation").dev(),
                                )
                                .unwrap()
                            });
                        }
                    }
                    _ => {}
                }
            }
            Event::Key(key, state) if *key == Key::Tab && *state == ButtonState::Pressed => {
                self.toggle_view();
            }
            _evt => {
                // println!("Event: {:#?}", _evt)
            }
        };
        #[cfg(feature = "dev_view")]
        self.dev_view_event(event);
        self.world.maintain();
        Ok(())
    }
}
