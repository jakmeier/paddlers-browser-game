use crate::game::town::DefaultShop;
use crate::game::Game;
use crate::gui::gui_components::ResourcesComponent;
use crate::gui::gui_components::TableTextProvider;
use crate::gui::input::{left_click::TownLeftClickSystem, MouseState};
use crate::gui::ui_state::UiState;
use crate::gui::utils::*;
use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use crate::resolution::ScreenResolution;
use crate::view::{ExperimentalSignalChannel, Frame};
use quicksilver::prelude::{MouseButton, Rectangle, Window};
use specs::prelude::*;

pub(crate) struct TownMenuFrame<'a, 'b> {
    text_provider: TableTextProvider,
    bank_component: ResourcesComponent,
    hover_component: ResourcesComponent,
    resources_area: Rectangle,
    left_click_dispatcher: Dispatcher<'a, 'b>,
}
impl TownMenuFrame<'_, '_> {
    pub fn new<'a, 'b>(game: &mut Game<'a, 'b>, ep: EventPool) -> PadlResult<Self> {
        let mut left_click_dispatcher = DispatcherBuilder::new()
            .with(TownLeftClickSystem::new(ep), "", &[])
            .build();
        left_click_dispatcher.setup(&mut game.world);

        Ok(TownMenuFrame {
            text_provider: TableTextProvider::new(),
            left_click_dispatcher,
            resources_area: Rectangle::default(),
            bank_component: ResourcesComponent::new()?,
            hover_component: ResourcesComponent::new()?,
        })
    }
}
impl<'a, 'b> Frame for TownMenuFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    type Signal = Signal;
    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        self.text_provider.reset();
        let inner_area = state.render_menu_box(window)?;
        let resolution = *state.world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        let entity = state.world.fetch::<UiState>().selected_entity;

        let (resources_area, menu_area) = inner_area.cut_horizontal(resources_height);
        self.resources_area = resources_area;
        self.bank_component.show()?;
        render_town_menu(
            state,
            window,
            entity,
            &menu_area,
            &mut self.text_provider,
            &mut self.hover_component,
        )?;
        self.text_provider.finish_draw();
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.text_provider.hide();
        self.bank_component.hide()?;
        self.hover_component.hide()?;
        Ok(())
    }
    fn left_click(
        &mut self,
        state: &mut Self::State,
        pos: (i32, i32),
        signals: &mut ExperimentalSignalChannel,
    ) -> Result<(), Self::Error> {
        state.click_buttons(pos);
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        std::mem::drop(ms); // This drop is essential! The internal RefCell will not be release otherwise
        self.left_click_dispatcher.dispatch(&state.world);
        // TODO: Only temporary experiment
        let mut result_signals = state.world.write_resource::<ExperimentalSignalChannel>();
        signals.append(&mut result_signals);
        Ok(())
    }
    fn event(&mut self, state: &mut Self::State, e: &Self::Event) -> Result<(), Self::Error> {
        match e {
            PadlEvent::Signal(Signal::ResourcesUpdated) => {
                self.bank_component
                    .draw(&self.resources_area, &state.resources.non_zero_resources())?;
            }
            PadlEvent::Signal(Signal::NewStoryState(s)) => {
                // FIXME: redundant with the same call also in dialogue
                state.set_story_state(*s);
                DefaultShop::reload(&mut state.world);
            }
            _ => {}
        }
        Ok(())
    }
}

fn render_town_menu(
    state: &mut Game<'_, '_>,
    window: &mut Window,
    entity: Option<Entity>,
    area: &Rectangle,
    text_provider: &mut TableTextProvider,
    hover_component: &mut ResourcesComponent,
) -> PadlResult<()> {
    match entity {
        Some(id) => {
            state.render_entity_details(window, area, id, text_provider, hover_component)?;
        }
        None => {
            state.render_default_shop(window, area, text_provider, hover_component)?;
        }
    }
    Ok(())
}
