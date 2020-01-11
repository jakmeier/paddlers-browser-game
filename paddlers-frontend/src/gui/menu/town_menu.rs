use crate::prelude::*;
use quicksilver::prelude::{Window, Rectangle, MouseButton};
use specs::prelude::*;
use crate::resolution::ScreenResolution;
use crate::game::Game;
use crate::gui::utils::*;
use crate::gui::ui_state::UiState;
use crate::gui::input::{
    left_click::TownLeftClickSystem,
    MouseState,
};
use crate::view::Frame;
use crate::gui::gui_components::ResourcesComponent;
use crate::init::quicksilver_integration::Signal;

pub (crate) struct TownMenuFrame<'a,'b> {
    pub text_pool: TextPool,
    bank_component: ResourcesComponent,
    hover_component: ResourcesComponent,
    resources_area: Rectangle,
    left_click_dispatcher: Dispatcher<'a,'b>,
}
impl TownMenuFrame<'_,'_> {
    pub fn new<'a,'b>(game: &mut Game<'a,'b>, ep: EventPool) -> PadlResult<Self> {

        let mut left_click_dispatcher = DispatcherBuilder::new()
            .with(TownLeftClickSystem::new(ep), "", &[])
            .build();
        left_click_dispatcher.setup(&mut game.world);

        Ok(TownMenuFrame {
            text_pool: TextPool::default(),
            left_click_dispatcher,
            resources_area: Rectangle::default(),
            bank_component: ResourcesComponent::new()?,
            hover_component: ResourcesComponent::new()?,
        })
    }
}
impl<'a,'b> Frame for TownMenuFrame<'a,'b> {
    type Error = PadlError;
    type State = Game<'a,'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    fn draw(&mut self, state: &mut Self::State, window: &mut Self::Graphics) -> Result<(),Self::Error> {
        self.text_pool.reset();
        let inner_area = state.render_menu_box(window)?;
        let resolution = *state.world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        let entity = state.world.fetch::<UiState>().selected_entity;
        
        let (resources_area, menu_area) = inner_area.cut_horizontal(resources_height);
        self.resources_area = resources_area;
        self.bank_component.show()?;
        render_town_menu(state, window, entity, &menu_area, &mut self.text_pool, &mut self.hover_component)?;
        self.text_pool.finish_draw();
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(),Self::Error> {
        self.text_pool.hide();
        self.bank_component.hide()?;
        self.hover_component.hide()?;
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32,i32)) -> Result<(),Self::Error> {
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        std::mem::drop(ms); // This drop is essential! The internal RefCell will not be release otherwise
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
    fn event(&mut self, state: &mut Self::State, e: &Self::Event) -> Result<(),Self::Error> {
        match e {
            PadlEvent::Signal(Signal::ResourcesUpdated) => {
                self.bank_component.draw(&self.resources_area, &state.resources.non_zero_resources())?;
            },
            _ => {}
        }
        Ok(())
    }
}

fn render_town_menu(
    state: &mut Game<'_,'_>,
    window: &mut Window,
    entity: Option<Entity>,
    area: &Rectangle,
    floats: &mut TextPool,
    hover_component: &mut ResourcesComponent,
) -> PadlResult<()> {
    match entity {
        Some(id) => {
            state.render_entity_details(window, area, id, floats, hover_component)?;
        },
        None => {
            state.render_default_shop(window, area, floats, hover_component)?;
        },
    }
    Ok(())
}