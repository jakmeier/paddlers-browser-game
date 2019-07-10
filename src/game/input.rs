use quicksilver::geom::{Vector, Shape, Rectangle};
use specs::prelude::*;
use specs::world::Index;
use crate::game::movement::Position;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub bool);

#[derive(Default, Clone, Copy)]
pub struct UiState {
    pub selected_entity: Option<Index>,
    pub hovered_entity: Option<Index>,
    pub menu_box_area: Rectangle,
}
pub struct MouseSystem;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

impl<'a> System<'a> for MouseSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
     );

    fn run(&mut self, (entities, mouse_state, mut ui_state, position, clickable): Self::SystemData) {
        let MouseState(mouse_pos, clicking) = *mouse_state;
        if mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area) {
            return;
        }

        (*ui_state).hovered_entity = None;
        if clicking {
            (*ui_state).selected_entity = None;
        }

        for (e, pos) in (&entities, &position).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                (*ui_state).hovered_entity = Some(e.id());
                let clickable: Option<&Clickable> = clickable.get(e);
                if clicking && clickable.is_some() {
                    (*ui_state).selected_entity = Some(e.id());
                }
            }
        }
    }

}