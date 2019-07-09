use quicksilver::geom::{Vector, Shape, Rectangle};
use specs::prelude::*;
use specs::world::Index;
use crate::game::movement::Position;

#[derive(Default, Clone, Copy)]
pub struct Click(pub Vector);

#[derive(Default, Clone, Copy)]
pub struct MenuBoxData {
    pub selected_entity: Option<Index>,
    pub area: Rectangle,
}
pub struct ClickSystem;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

impl<'a> System<'a> for ClickSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Click>,
        Write<'a, MenuBoxData>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
     );

    fn run(&mut self, (entities, click, mut menu_box_data, position, clickable): Self::SystemData) {
        let click = click.0;
        if click.overlaps_rectangle(&(*menu_box_data).area) {
            return;
        }
        (*menu_box_data).selected_entity = None;
        for (e, pos, _) in (&entities, &position, &clickable).join() {
            if click.overlaps_rectangle(&pos.area) {
                // For now, clickable <=> selectable
                (*menu_box_data).selected_entity = Some(e.id());
            }
        }
    }

}