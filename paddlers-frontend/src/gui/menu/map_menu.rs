use crate::game::Game;
use crate::gui::{
    gui_components::{ResourcesComponent, TableTextProvider},
    input::left_click::MapLeftClickSystem,
    input::MouseState,
    menu::*,
    ui_state::UiState,
};
use crate::prelude::*;
use paddle::Frame;
use quicksilver::prelude::{MouseButton, Shape, Window};
use specs::prelude::*;

pub(crate) struct MapMenuFrame<'a, 'b> {
    text_provider: TableTextProvider,
    left_click_dispatcher: Dispatcher<'a, 'b>,
    _hover_component: ResourcesComponent,
}
impl MapMenuFrame<'_, '_> {
    pub fn new() -> PadlResult<Self> {
        let left_click_dispatcher = DispatcherBuilder::new()
            .with(MapLeftClickSystem::new(), "", &[])
            .build();

        Ok(MapMenuFrame {
            text_provider: TableTextProvider::new(),
            left_click_dispatcher,
            _hover_component: ResourcesComponent::new()?,
        })
    }
}
impl<'a, 'b> Frame for MapMenuFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;

    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        self.text_provider.reset();
        let inner_area = state.inner_menu_area();

        let selected_entity = state.world.fetch::<UiState>().selected_entity;
        if let Some(e) = selected_entity {
            let (img_area, table_area) = menu_selected_entity_spacing(&inner_area);
            let world = &state.world;
            let sprites = &mut state.sprites;
            draw_entity_img(world, sprites, window, e, &img_area)?;
            draw_map_entity_details_table(
                world,
                sprites,
                window,
                e,
                &table_area,
                &mut self.text_provider,
            )?;
        }
        self.text_provider.finish_draw();
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        // This can be removed once the frame positions are checked properly before right_click is called
        let ui_state = state.world.fetch::<ViewState>();
        let mouse_pos: Vector = pos.into();
        let in_menu_area = mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area);
        if !in_menu_area {
            return Ok(());
        }
        std::mem::drop(ui_state);

        let ms = MouseState(mouse_pos, Some(MouseButton::Left));
        state.world.insert(ms);
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.text_provider.hide();
        Ok(())
    }
}
