use crate::gui::{
    gui_components::{ResourcesComponent, TableTextProvider},
    input::left_click::MapLeftClickSystem,
    input::MouseState,
    menu::*,
    ui_state::UiState,
};
use crate::prelude::*;
use crate::{game::Game, gui::input::MouseButton};
use paddle::Frame;
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
    type State = Game;
    const WIDTH: u32 = crate::resolution::MENU_AREA_W;
    const HEIGHT: u32 = crate::resolution::MENU_AREA_H;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.text_provider.reset();
        let inner_area = crate::gui::menu::inner_menu_area();

        let selected_entity = state.world.fetch::<UiState>().selected_entity;
        if let Some(e) = selected_entity {
            let (img_area, table_area) = menu_selected_entity_spacing(&inner_area);
            let world = &state.world;
            let sprites = &mut state.sprites;
            entity_details::draw_entity_img(world, sprites, window, e, &img_area);
            entity_details::draw_map_entity_details_table(
                world,
                sprites,
                window,
                e,
                &table_area,
                &mut self.text_provider,
                state.mouse.pos(),
            );
        }
        self.text_provider.finish_draw();
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let mouse_pos: Vector = pos.into();
        let ms = MouseState(mouse_pos, Some(MouseButton::Left));
        state.world.insert(ms);
        self.left_click_dispatcher.dispatch(&state.world);
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.text_provider.hide();
    }
}
