use super::*;
use crate::game::Game;
use crate::prelude::*;
use paddle::quicksilver_compat::geom::Shape;
use paddle::Frame;
use paddle::Window;
use std::marker::PhantomData;

pub(crate) struct MapFrame<'a, 'b> {
    phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> MapFrame<'a, 'b> {
    pub fn new() -> Self {
        MapFrame {
            phantom: PhantomData,
        }
    }
}

impl<'a, 'b> Frame for MapFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;

    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        let ui_state = state.world.read_resource::<ViewState>();
        let area = Rectangle::new(
            (0, 0),
            (
                ui_state.menu_box_area.x(),
                (window.project() * window.screen_size()).y,
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
