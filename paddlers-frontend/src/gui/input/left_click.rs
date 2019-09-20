use super::{Clickable, MouseState, UiState, UiView};
use crate::game::{
    components::*,
    map::{GlobalMapSharedState, MapPosition},
    movement::*,
    town::{town_shop::DefaultShop, Town},
    town_resources::TownResources,
    units::workers::*,
    abilities::*,
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
        ReadStorage<'a, Moving>,
        WriteStorage<'a, EntityContainer>,
        WriteStorage<'a, UiMenu>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, Health>,
    );

    fn run(
        &mut self,
        (
            entities,
            mouse_state,
            mut ui_state,
            shop,
            buttons,
            mut resources,
            mut town,
            mut map,
            mut rest,
            mut errq,
            lazy,
            position,
            map_position,
            clickable,
            moving,
            containers,
            mut ui_menus,
            mut workers,
            mut health,
        ): Self::SystemData,
    ) {
        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Left) {
            return;
        }

        let active_entity = ui_state.selected_entity;

        // Always visible buttons
        buttons.click(mouse_pos, &mut *ui_state);

        // Demultiplex signal to views
        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        match (ui_state.current_view, in_menu_area) {
            (UiView::Town, true) => {
                if let Some(entity) = (*ui_state).selected_entity {
                    if let Some(ui_menu) = ui_menus.get_mut(entity) {
                        town.left_click_on_menu(
                            entity, mouse_pos, ui_state, position, workers, containers, ui_menu, lazy, errq, rest,
                        );
                    }
                }
                else {
                    Town::click_default_shop(mouse_pos, ui_state, shop, resources);
                }
            }
            (UiView::Map, true) => {
                // NOP
            }
            (UiView::Town, false) => {
                let maybe_ability =
                    town.left_click(
                        mouse_pos, &entities, &mut ui_state, &position, &clickable, &lazy, &mut resources, &mut errq, &mut rest,
                    );
                match maybe_ability {
                    Some(Ability::Walk) => {
                        let active_entity = active_entity.expect("Ability requires unit");
                        let worker = workers.get_mut(active_entity).expect("Ability requires unit");
                        worker.new_order(
                            active_entity, 
                            &*town,
                            mouse_pos,
                            &mut *rest,
                            &mut *errq,
                            &position,
                            &moving,
                            &containers,
                            &entities,
                        );

                    },
                    Some(Ability::Welcome) => {
                        use_welcome_ability(
                            mouse_pos, 
                            &mut *rest,
                            &mut *errq,
                            &position,
                            &clickable,
                            &mut health,
                            &entities,
                        );
                    },
                    None => {},
                }
            }
            (UiView::Map, false) => map.left_click_on_main_area(mouse_pos, ui_state, entities, map_position, clickable),
        }
    }
}