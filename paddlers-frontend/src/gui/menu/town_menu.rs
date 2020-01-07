use crate::gui::input::Grabbable;
use crate::prelude::*;
use quicksilver::prelude::{Window, Rectangle, Event, MouseButton};
use specs::prelude::*;
use crate::resolution::ScreenResolution;
use crate::game::Game;
use crate::gui::utils::*;
use crate::gui::input::{
    left_click::TownLeftClickSystem,
    MouseState,
};
use crate::view::Frame;

pub (crate) struct TownMenuFrame<'a,'b> {
    pub text_pool: TextPool,
    pub selected_entity: Option<Entity>,
    pub hovered_entity: Option<Entity>,
    pub grabbed_item: Option<Grabbable>,
    left_click_dispatcher: Dispatcher<'a,'b>
}
impl TownMenuFrame<'_,'_> {
    pub fn new<'a,'b>(game: &mut Game<'a,'b>, ep: EventPool) -> PadlResult<Self> {

        let mut left_click_dispatcher = DispatcherBuilder::new()
            .with(TownLeftClickSystem::new(ep), "", &[])
            .build();
        left_click_dispatcher.setup(&mut game.world);

        Ok(TownMenuFrame {
            text_pool: TextPool::default(),
            selected_entity: None,
            hovered_entity: None,
            grabbed_item: None,
            left_click_dispatcher,
        })
    }
}
impl<'a,'b> Frame for TownMenuFrame<'a,'b> {
    type Error = PadlError;
    type State = Game<'a,'b>;
    type Graphics = Window;
    type Event = Event;
    fn draw(&mut self, state: &mut Self::State, window: &mut Self::Graphics) -> Result<(),Self::Error> {
        self.text_pool.reset();
        let inner_area = state.render_menu_box(window)?;
        let resolution = *state.world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        let entity = self.selected_entity;
        
        let (resources_area, menu_area) = inner_area.cut_horizontal(resources_height);
        state.render_resources(window, &resources_area, &mut self.text_pool)?;
        render_town_menu(state, window, entity, &menu_area, &mut self.text_pool)?;
        self.text_pool.finish_draw();
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(),Self::Error> {
        self.text_pool.hide();
        Ok(())
    }
    fn update(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        //TODO
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32,i32)) -> Result<(),Self::Error> {
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        std::mem::drop(ms); // This drop is essential! The internal RefCell will not be release otherwise
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
}

fn render_town_menu(
    state: &mut Game<'_,'_>,
    window: &mut Window,
    entity: Option<Entity>,
    area: &Rectangle,
    floats: &mut TextPool,
) -> PadlResult<()> {
    match entity {
        Some(id) => {
            state.render_entity_details(window, area, id, floats)?;
        },
        None => {
            state.render_default_shop(window, area, floats)?;
        },
    }
    Ok(())
}