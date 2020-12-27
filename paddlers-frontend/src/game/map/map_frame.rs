use super::*;
use crate::{
    game::Game,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use paddle::quicksilver_compat::geom::Shape;
use paddle::{DisplayArea, Frame};

pub(crate) struct MapFrame {
    mouse: PointerTracker,
}

impl MapFrame {
    pub fn new() -> Self {
        MapFrame {
            mouse: PointerTracker::new(),
        }
    }
    fn left_click(&mut self, state: &mut Game, pos: Vector) {
        let mut map = state.world.fetch_mut::<GlobalMapSharedState>();

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
    fn drag(&mut self, state: &mut Game, (start, end): (Vector, Vector)) {
        let mut map = state.world.write_resource::<GlobalMapSharedState>();
        let v = end - start;
        map.drag(v * 0.02);
    }
}

impl Frame for MapFrame {
    type State = Game;
    const WIDTH: u32 = MAIN_AREA_W;
    const HEIGHT: u32 = MAIN_AREA_H;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        let (sprites, mut map) = (
            &mut state.sprites,
            GlobalMap::combined(
                state.map.as_mut().expect("map"),
                state.world.write_resource(),
            ),
        );
        window.fill(Col(GREEN));

        // self.apply_scaling(area.size());
        map.draw_grid(window);
        map.draw_water(window, &Self::area());
        map.draw_villages(window, sprites);
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        if let PointerEvent(PointerEventType::PrimaryClick, pos) = event {
            self.left_click(state, pos)
        }
        if let Some(drag) = self.mouse.take_drag() {
            self.drag(state, drag);
        }
    }
}
