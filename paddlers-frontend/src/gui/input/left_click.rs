use super::{Clickable, MouseState, UiState, UiView};
use crate::game::{
    components::*,
    map::{GlobalMapSharedState, MapPosition},
    movement::*,
    town::{town_shop::DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
};
use crate::gui::menu::buttons::MenuButtons;
use crate::logging::ErrorQueue;
use crate::net::game_master_api::RestApiState;
use quicksilver::prelude::*;
use specs::prelude::*;

pub struct LeftClickSystem;

impl<'a> System<'a> for LeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, DefaultShop>,
        ReadExpect<'a, MenuButtons>,
        Write<'a, TownResources>,
        Write<'a, Town>,
        Write<'a, GlobalMapSharedState>,
        WriteExpect<'a, RestApiState>,
        WriteExpect<'a, ErrorQueue>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, MapPosition>,
        ReadStorage<'a, Clickable>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, Worker>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut ui_state,
            shop,
            buttons,
            resources,
            mut town,
            mut map,
            rest,
            errq,
            lazy,
            position,
            map_position,
            clickable,
            containers,
            workers,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Left) {
            return;
        }

        // Always visible buttons
        buttons.click(mouse_pos, &mut *ui_state);

        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        match (ui_state.current_view, in_menu_area) {
            (UiView::Town, true) => {
                town.left_click_on_menu(
                    mouse_pos, ui_state, position, workers, containers, lazy, shop, resources,
                    errq, rest,
                );
            }
            (UiView::Map, true) => {
                // NOP
            }
            (UiView::Town, false) => {
                town.left_click(
                    mouse_pos, entities, ui_state, position, clickable, lazy, resources, errq, rest,
                );
            }
            (UiView::Map, false) => map.left_click_on_main_area(mouse_pos, ui_state, entities, map_position, clickable),
        }
    }
}
