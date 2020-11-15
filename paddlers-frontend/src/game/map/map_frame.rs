use super::*;
use crate::game::Game;
use crate::prelude::*;
use paddle::quicksilver_compat::geom::Shape;
use paddle::Frame;
use paddle::WebGLCanvas;

pub(crate) struct MapFrame {}

impl MapFrame {
    pub fn new() -> Self {
        MapFrame {}
    }
}

impl Frame for MapFrame {
    type Error = PadlError;
    type State = Game;

    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut WebGLCanvas,
        _timestamp: f64,
    ) -> Result<(), Self::Error> {
        let ui_state = state.world.read_resource::<ViewState>();
        let area = Rectangle::new(
            (0, 0),
            (
                ui_state.menu_box_area.x(),
                (window.project() * window.browser_region().size()).y,
            ),
        );
        let (sprites, mut map) = (
            &mut state.sprites,
            GlobalMap::combined(
                state.map.as_mut().expect("map"),
                state.world.write_resource(),
            ),
        );
        map.render(window, sprites, &area);
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        let mut map = state.world.fetch_mut::<GlobalMapSharedState>();

        let pos: Vector = pos.into();
        let main_area = state.world.read_resource::<ViewState>().main_area;
        if pos.overlaps_rectangle(&main_area) {
            map.left_click_on_main_area(
                pos,
                &mut *state.world.write_resource(),
                state.world.entities(),
                state.world.read_storage(),
                state.world.read_storage(),
            );
        }
        Ok(())
    }
}
