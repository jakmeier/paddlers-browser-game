use crate::prelude::*;
use crate::{game::fight::*, net::game_master_api::HttpDeleteBuilding};
use crate::{game::movement::Position, net::game_master_api::RestApiState};
use crate::{game::town::tiling, net::state::current_village};
/// This module keeps the logic to read input and, in most cases,
/// redirect it to suitable modules to handle the input
use paddle::quicksilver_compat::*;
use paddle::Vector;
use paddlers_shared_lib::prelude::*;
use specs::prelude::*;

pub mod left_click;
pub use self::left_click::*;
use crate::gui::ui_state::UiState;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UiView {
    Visitors(VisitorViewTab),
    Leaderboard,
    Map,
    Town,
    TownHelp,
    Dialogue,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VisitorViewTab {
    IncomingAttacks,
    Letters,
    Quests,
}

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone, Debug)]
pub enum Grabbable {
    NewBuilding(BuildingType),
    Ability(AbilityType),
}

impl crate::game::Game {
    pub fn hotkey(&mut self, key: Key) {
        match key {
            Key::Escape => {
                let mut ui_state = self.world.write_resource::<UiState>();
                if ui_state.take_grabbed_item().is_none() {
                    ui_state.selected_entity = None;
                }
                let mut ui_state = self.town_world().write_resource::<UiState>();
                if ui_state.take_grabbed_item().is_none() {
                    ui_state.selected_entity = None;
                }
            }
            Key::Delete => {
                let view = *self.world.fetch::<UiView>();
                let town_world = self.town_world();
                let mut ui_state = town_world.write_resource::<UiState>();
                match view {
                    UiView::Town => {
                        if let Some(e) = ui_state.selected_entity {
                            (*ui_state).selected_entity = None;

                            std::mem::drop(ui_state);

                            let pos_store = town_world.read_storage::<Position>();
                            let pos = pos_store.get(e).unwrap();
                            let tile_index = tiling::tile(pos.area.center());
                            std::mem::drop(pos_store);

                            nuts::send_to::<RestApiState, _>(HttpDeleteBuilding {
                                idx: tile_index,
                                village: current_village(),
                            });

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
            Key::Tab => {
                self.toggle_view();
            }
            Key::KeyH => {
                self.toggle_help_view();
            }
            _key => {
                // println!("Key: {:#?}", _evt)
            }
        };
        #[cfg(feature = "dev_view")]
        self.dev_view_hotkey(key);
        self.world.maintain();
    }
}
