use crate::net::game_master_api::RestApiState;
use quicksilver::geom::{Vector, Shape, Rectangle};
use quicksilver::prelude::MouseButton;
use specs::prelude::*;
use crate::gui::{
    utils::*,
    sprites::WithSprite,
    gui_components::*,
};
use crate::game::{
    movement::*,
    town_resources::TownResources,
    town::Town,
    units::workers::Worker,
    components::*,
};
use paddlers_shared_lib::models::*;
use paddlers_shared_lib::api::shop::*;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub Option<MouseButton>);

#[derive(Default, Clone)]
pub struct UiState {
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    pub menu_box_area: Rectangle,
}
pub struct LeftClickSystem;
pub struct RightClickSystem;
pub struct HoverSystem;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone)]
pub enum Grabbable {
    NewBuilding(BuildingType),
}

impl<'a> System<'a> for LeftClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, DefaultShop>,
        Write<'a, TownResources>,
        Write<'a, Town>,
        Write<'a, RestApiState>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
     );

    fn run(&mut self, (entities, mouse_state, mut ui_state, shop, mut resources, mut town, mut rest, lazy, position, clickable): Self::SystemData) {

        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Left) {
            return;
        }
        
        if mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area) {
            if (*ui_state).selected_entity.is_none() {
                let maybe_grab = shop.click(mouse_pos);
                if let Some(Grabbable::NewBuilding(b)) = maybe_grab {
                    if (*resources).can_afford(&b.price()) {
                        (*ui_state).grabbed_item = maybe_grab;
                    }
                }
            }
        }
        else {
            (*ui_state).selected_entity = None;
            for (e, pos, _) in (&entities, &position, &clickable).join() {
                if mouse_pos.overlaps_rectangle(&pos.area) {
                    (*ui_state).selected_entity = Some(e);
                    break;
                }
            }
            if let Some(grabbed) = &(*ui_state).grabbed_item {
                match grabbed {
                    Grabbable::NewBuilding(bt) => {
                        if let Some(pos) = (*town).get_buildable_tile(mouse_pos) {
                            rest.http_place_building(pos, *bt);
                            resources.spend(&bt.price());
                            town.insert_new_bulding(&entities, &lazy, pos, *bt);
                            (*ui_state).grabbed_item = None;
                        }
                    },
                }
            }
        }

    }
}

impl<'a> System<'a> for RightClickSystem {
    type SystemData = (
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, Town>,
        Write<'a, RestApiState>,
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, NetObj>,
        ReadStorage<'a, Moving>,
     );

    fn run(&mut self, (mouse_state, mut ui_state, town, mut rest, entities, mut worker, position, netobj, moving): Self::SystemData) {

        let MouseState(mouse_pos, button) = *mouse_state;
        if button != Some(MouseButton::Right) {
            return;
        }

        (*ui_state).grabbed_item = None;


        if mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area) {
            // NOP
        }
        else {
            if let Some((worker, from, netid, movement)) = 
                ui_state.selected_entity
                .and_then(
                    |selected| 
                    (&mut worker, &position, &netobj, &moving).join().get(selected, &entities) 
                )
            {
                let start = town.next_tile_in_direction(from.area.pos, movement.momentum);
                let destination = town.tile(mouse_pos);
                let msg = worker.new_walk_task(start , destination , &town, netid.id);
                match msg {
                    Ok(msg) => {
                        rest.http_overwrite_tasks(msg);
                    }
                    Err(e) => {
                        println!("Walking didn't work: {}", e);
                    }
                }
            }
        }
    }
}

impl<'a> System<'a> for HoverSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        ReadStorage<'a, Position>,
     );

    fn run(&mut self, (entities, mouse_state, mut ui_state, position): Self::SystemData) {

        let MouseState(mouse_pos, _) = *mouse_state;
        
        (*ui_state).hovered_entity = None;

        for (e, pos) in (&entities, &position).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                (*ui_state).hovered_entity = Some(e);
                break;
            }
        }
    }
}

// TODO: Eventually, this should be split up between different buildings
#[derive(Clone)]
pub struct DefaultShop {
    pub ui: UiBox<BuildingType>,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui : UiBox::new(Rectangle::default(), 3, 5)
        }
    }
}
impl DefaultShop {
    pub fn new(area: Rectangle) -> Self {
        let mut result = DefaultShop {
            ui : UiBox::new(area, 3, 5)
        };
        result.add_building(BuildingType::BlueFlowers);
        result.add_building(BuildingType::RedFlowers);
        result.add_building(BuildingType::Tree);
        result
    }

    fn add_building(&mut self, b: BuildingType) {
        self.ui.add_with_background_color_and_cost(b.sprite(), WHITE, b, b.cost());
    }

    fn click(&self, mouse: impl Into<Vector>) -> Option<Grabbable> {
        let buy_this = self.ui.click(mouse);
        if let Some(building_type) = buy_this {
            return Some(
                Grabbable::NewBuilding(building_type)
            )
        }
        None
    }
}