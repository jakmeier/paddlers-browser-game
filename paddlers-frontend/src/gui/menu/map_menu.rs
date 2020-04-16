use crate::prelude::*;
use quicksilver::prelude::{Window, MouseButton};
use specs::prelude::*;
use crate::game::Game;
use crate::gui::gui_components::TableTextProvider;
use crate::gui::ui_state::UiState;
use crate::gui::input::{
    left_click::MapLeftClickSystem,
    MouseState,
};
use crate::view::Frame;
use crate::gui::gui_components::ResourcesComponent;

pub (crate) struct MapMenuFrame<'a,'b> {
    text_provider: TableTextProvider,
    left_click_dispatcher: Dispatcher<'a,'b>,
    _hover_component: ResourcesComponent,
}
impl MapMenuFrame<'_,'_> {
    pub fn new<'a,'b>(game: &mut Game<'a,'b>, ep: EventPool) -> PadlResult<Self> {

        let mut left_click_dispatcher = DispatcherBuilder::new()
            .with(MapLeftClickSystem::new(ep), "", &[])
            .build();
        left_click_dispatcher.setup(&mut game.world);

        Ok(MapMenuFrame {
            text_provider: TableTextProvider::new(),
            left_click_dispatcher,
            _hover_component: ResourcesComponent::new()?,
        })
    }
}
impl<'a,'b> Frame for MapMenuFrame<'a,'b> {
    type Error = PadlError;
    type State = Game<'a,'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    fn draw(&mut self, state: &mut Self::State, window: &mut Self::Graphics) -> Result<(),Self::Error> {
        self.text_provider.reset();
        let inner_area = state.render_menu_box(window)?;
        
        let selected_entity = state.world.fetch::<UiState>().selected_entity;
        if let Some(e) = selected_entity {
            state.render_entity_details(
                window,
                &inner_area,
                e,
                &mut self.text_provider,
                &mut self._hover_component,
            )?;
        }
        self.text_provider.finish_draw();
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32,i32)) -> Result<(),Self::Error> {
        state.click_buttons(pos);
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        std::mem::drop(ms); // This drop is essential! The internal RefCell will not be release otherwise
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.text_provider.hide();
        Ok(())
    }
}