use crate::prelude::*;
use quicksilver::prelude::{Window, Event, MouseButton};
use specs::prelude::*;
use crate::view::FloatingText;
use crate::resolution::ScreenResolution;
use crate::game::{
    Game,
};
use crate::gui::input::{
    left_click::MapLeftClickSystem,
    MouseState,
};
use crate::view::Frame;

pub (crate) struct MapMenuFrame<'a,'b> {
    pub selected_entity: Option<Entity>,
    floats: [FloatingText;3],
    left_click_dispatcher: Dispatcher<'a,'b>
}
impl MapMenuFrame<'_,'_> {
    pub fn new<'a,'b>(game: &mut Game<'a,'b>, ep: EventPool) -> PadlResult<Self> {

        let mut left_click_dispatcher = DispatcherBuilder::new()
            .with(MapLeftClickSystem::new(ep), "", &[])
            .build();
        left_click_dispatcher.setup(&mut game.world);

        Ok(MapMenuFrame {
            selected_entity: None,
            floats: FloatingText::new_triplet()?,
            left_click_dispatcher
        })
    }
}
impl<'a,'b> Frame for MapMenuFrame<'a,'b> {
    type Error = PadlError;
    type State = Game<'a,'b>;
    type Graphics = Window;
    type Event = Event;
    fn draw(&mut self, state: &mut Self::State, window: &mut Self::Graphics) -> Result<(),Self::Error> {
        let inner_area = state.render_menu_box(window)?;
        let resolution = *state.world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        
        if let Some(e) = self.selected_entity {
            state.render_entity_details(window, &inner_area, e, &mut self.floats)?;
        }
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32,i32)) -> Result<(),Self::Error> {
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
}