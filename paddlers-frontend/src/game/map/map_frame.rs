use super::*;
use crate::{
    game::Game,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use paddle::quicksilver_compat::geom::Shape;
use paddle::{DisplayArea, Frame};

pub(crate) struct MapFrame {}

impl MapFrame {
    pub fn new() -> Self {
        MapFrame {}
    }
}

impl Frame for MapFrame {
    type State = Game;
    const WIDTH: u32 = MAIN_AREA_W;
    const HEIGHT: u32 = MAIN_AREA_H;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        let area = Rectangle::new((0, 0), Self::size());
        let (sprites, mut map) = (
            &mut state.sprites,
            GlobalMap::combined(
                state.map.as_mut().expect("map"),
                state.world.write_resource(),
            ),
        );
        map.render(window, sprites, &area);
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let mut map = state.world.fetch_mut::<GlobalMapSharedState>();

        let pos: Vector = pos.into();
        let main_area = Rectangle::new_sized((MAIN_AREA_W, MAIN_AREA_H));
        if pos.overlaps_rectangle(&main_area) {
            map.left_click_on_main_area(
                pos,
                &mut *state.world.write_resource(),
                state.world.entities(),
                state.world.read_storage(),
                state.world.read_storage(),
            );
        }
    }
    // TODO: dragging
    // fn mouse_move(&mut self, state: &mut Self::State, pos: (i32, i32)) {
    //     let v = end - start;
    //     map.drag(v * 0.02);
    // }
}

// TODO
// #[derive(Default, Clone, Copy)]
// /// Represents a drag input waiting to be processed by the DragSystem.
// /// Can only hold one drag at the time.
// /// When more drags are added, they are summarized to one single movement.
// pub struct Drag(Option<(Vector, Vector)>);

// impl Drag {
//     // THIS NEEDS INTEGRATION
//     pub fn add(&mut self, start: Vector, end: Vector) {
//         if let Some(old) = self.0 {
//             self.0 = Some((old.0, end));
//         } else {
//             self.0 = Some((start, end));
//         }
//     }
//     pub fn is_some(&self) -> bool {
//         self.0.is_some()
//     }
//     pub fn clear(&mut self) {
//         self.0 = None;
//     }
// }
