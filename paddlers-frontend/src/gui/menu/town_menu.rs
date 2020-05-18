use crate::game::town::DefaultShop;
use crate::game::town_resources::TownResources;
use crate::game::Game;
use crate::gui::{
    gui_components::{ResourcesComponent, TableTextProvider},
    input::{left_click::TownMenuLeftClickSystem, MouseState},
    menu::*,
    ui_state::UiState,
    utils::*,
};
use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use crate::resolution::ScreenResolution;
use crate::view::{ExperimentalSignalChannel, Frame};
use quicksilver::prelude::{MouseButton, Rectangle, Shape, Window};
use specs::prelude::*;

pub(crate) struct TownMenuFrame<'a, 'b> {
    text_provider: TableTextProvider,
    bank_component: ResourcesComponent,
    hover_component: ResourcesComponent,
    resources_area: Rectangle,
    left_click_dispatcher: Dispatcher<'a, 'b>,
}
impl TownMenuFrame<'_, '_> {
    pub fn new<'a, 'b>(ep: EventPool) -> PadlResult<Self> {
        let left_click_dispatcher = DispatcherBuilder::new()
            .with(TownMenuLeftClickSystem::new(ep), "", &[])
            .build();

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
        let inner_area = state.inner_menu_area();

        let world = state.town_world();
        let resolution = *world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        let entity = world.fetch::<UiState>().selected_entity;

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
        let town_world = state.town_world();

        // This can be removed once the frame positions are checked properly before right_click is called
        let ui_state = town_world.fetch_mut::<ViewState>();
        let mouse_pos: Vector = pos.into();
        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        if !in_menu_area {
            return Ok(());
        }
        std::mem::drop(ui_state);

        let ms = MouseState(pos.into(), Some(MouseButton::Left));
        state.town_world_mut().insert(ms);
        self.left_click_dispatcher.dispatch(state.town_world());
        // TODO: Only temporary experiment
        let mut result_signals = state
            .town_world()
            .write_resource::<ExperimentalSignalChannel>();
        signals.append(&mut result_signals);
        Ok(())
    }
    fn right_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        let town_world = state.town_world();

        // This can be removed once the frame positions are checked properly before right_click is called
        let view_state = town_world.fetch_mut::<ViewState>();
        let mouse_pos: Vector = pos.into();
        let in_menu_area = mouse_pos.overlaps_rectangle(&(*view_state).menu_box_area);
        if !in_menu_area {
            return Ok(());
        }
        // Right click cancels grabbed item (take removes from option)
        let mut ui_state = town_world.fetch_mut::<UiState>();
        ui_state.take_grabbed_item();
        Ok(())
    }
    fn event(&mut self, state: &mut Self::State, e: &Self::Event) -> Result<(), Self::Error> {
        match e {
            PadlEvent::Signal(Signal::ResourcesUpdated) => {
                self.bank_component.draw(
                    &self.resources_area,
                    &state
                        .town_world()
                        .fetch::<TownResources>()
                        .non_zero_resources(),
                )?;
            }
            PadlEvent::Signal(Signal::NewStoryState(s)) => {
                // FIXME: redundant with the same call also in dialogue
                state.set_story_state(*s);
                DefaultShop::reload(state.town_world_mut());
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
            let (img_area, table_area) = menu_selected_entity_spacing(&area);
            let world = state.town_context.world();
            let sprites = &mut state.sprites;
            draw_entity_img(world, sprites, window, id, &img_area)?;
            draw_town_entity_details_table(
                world,
                sprites,
                window,
                id,
                &table_area,
                text_provider,
                hover_component,
            )?;
        }
        None => {
            state.render_default_shop(window, area, text_provider, hover_component)?;
        }
    }
    Ok(())
}
