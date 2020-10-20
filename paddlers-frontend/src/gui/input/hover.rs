use super::{MouseState, UiView};
use crate::game::movement::*;
use crate::gui::ui_state::UiState;
use paddle::quicksilver_compat::Shape;
use specs::prelude::*;

pub struct HoverSystem;

impl<'a> System<'a> for HoverSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        WriteExpect<'a, UiState>,
        ReadExpect<'a, UiView>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (entities, mouse_state, mut ui_state, view, position): Self::SystemData) {
        let MouseState(mouse_pos, _) = *mouse_state;

        (*ui_state).hovered_entity = None;

        match *view {
            UiView::Map => {}
            UiView::Town => {
                for (e, pos) in (&entities, &position).join() {
                    if mouse_pos.overlaps_rectangle(&pos.area) {
                        (*ui_state).hovered_entity = Some(e);
                        break;
                    }
                }
            }
            UiView::Visitors(_) => {}
            UiView::Leaderboard => {}
            UiView::Dialogue => {}
        }
    }
}
