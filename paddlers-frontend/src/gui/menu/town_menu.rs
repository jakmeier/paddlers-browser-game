use crate::game::{
    game_event_manager::EventManager, toplevel::Signal, town::DefaultShop,
    town_resources::TownResources, Game,
};
use crate::gui::{
    gui_components::{ResourcesComponent, TableTextProvider, UiElement},
    input::{left_click::TownMenuLeftClickSystem, MouseState},
    menu::*,
    ui_state::UiState,
    utils::*,
};
use crate::prelude::*;
use chrono::NaiveDateTime;
use paddle::{
    nuts, DisplayArea, ErrorMessage, Frame, NutsCheck, PointerEvent, PointerEventType,
    PointerTracker, Rectangle, Vector,
};
use specs::prelude::*;

use super::entity_details::*;

pub(crate) struct TownMenuFrame<'a, 'b> {
    text_provider: TableTextProvider,
    bank_component: ResourcesComponent,
    hover_component: ResourcesComponent,
    foreign_town_menu: UiBox,
    left_click_dispatcher: Dispatcher<'a, 'b>,
    mouse: PointerTracker,
    html_attached: bool,
}

impl<'a, 'b> Frame for TownMenuFrame<'a, 'b> {
    type State = Game;
    const WIDTH: u32 = crate::gui::menu::INNER_MENU_AREA_W as u32;
    const HEIGHT: u32 = crate::gui::menu::INNER_MENU_AREA_H as u32;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        if !self.html_attached {
            self.bank_component.attach(window);
            self.hover_component.attach(window);
            self.html_attached = true;
        }

        self.text_provider.reset();
        let world = state.town_context.world();
        let mut area = Self::area();
        let resources_height = RESOURCES_H;
        let foreign = state.town_context.is_foreign();
        let now = world.fetch::<Now>().0;
        let selected_entity = world.fetch::<UiState>().selected_entity;

        if foreign {
            self.bank_component.clear();
            let extras_h = area.height() / 3.0;
            let (extras_area, remainder) = area.cut_horizontal(extras_h);
            area = remainder;
            self.render_foreign_town_extras(
                &mut state.sprites,
                window,
                &extras_area,
                now,
                self.mouse.pos(),
            );
        } else {
            let (top, remainder) = area.cut_horizontal(resources_height);
            let resources_area = top;
            area = remainder;
            let resources = &state
                .town_world()
                .fetch::<TownResources>()
                .non_zero_resources();
            self.bank_component.update(resources).nuts_check();
            // TODO: Calling this every frame is expensive
            self.bank_component
                .draw(window, &resources_area)
                .nuts_check();
        }

        if let Some(selected_entity) = selected_entity {
            let (img_area, table_area) = menu_selected_entity_spacing(&area);
            draw_entity_img(
                world,
                &mut state.sprites,
                window,
                selected_entity,
                &img_area,
            );
            draw_town_entity_details_table(
                world,
                &mut state.sprites,
                window,
                selected_entity,
                &table_area,
                &mut self.text_provider,
                &mut self.hover_component,
                self.mouse.pos(),
                foreign,
            );
        } else if !foreign {
            DefaultShop::render_default_shop(
                state,
                window,
                &area,
                &mut self.text_provider,
                &mut self.hover_component,
                self.mouse.pos(),
            );
        }

        self.text_provider.finish_draw();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.text_provider.hide();
        // self.hover_component.clear(); // that doesn't work? Doing it every frame instead...
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        self.mouse.track_pointer_event(&event);
        match event {
            PointerEvent(PointerEventType::PrimaryClick, pos) => self.left_click(state, pos),
            PointerEvent(PointerEventType::SecondaryClick, pos) => self.right_click(state, pos),
            _ => { /* NOP */ }
        }
    }
}
impl TownMenuFrame<'_, '_> {
    pub fn new<'a, 'b>() -> PadlResult<Self> {
        let left_click_dispatcher = DispatcherBuilder::new()
            .with(TownMenuLeftClickSystem::new(), "", &[])
            .build();

        let mut foreign_town_menu = UiBox::new(1, 1, 1.0, 1.0);
        foreign_town_menu.add(
            UiElement::new(ClickOutput::Event(GameEvent::LoadHomeVillage))
                .with_text("Go Home".to_owned())
                .with_background_color(LIGHT_BLUE),
        );

        Ok(TownMenuFrame {
            text_provider: TableTextProvider::new(),
            left_click_dispatcher,
            bank_component: ResourcesComponent::new()?,
            hover_component: ResourcesComponent::new()?,
            foreign_town_menu,
            mouse: Default::default(),
            html_attached: false,
        })
    }
    fn left_click(&mut self, state: &mut Game, pos: Vector) {
        let foreign = state.town_context.is_foreign();
        if foreign {
            if let Some((click_output, _condition)) = self.foreign_town_menu.click(pos) {
                match click_output {
                    ClickOutput::Event(evt) => {
                        nuts::send_to::<EventManager, _>(evt);
                    }
                    _ => {
                        nuts::publish(ErrorMessage::technical(
                            "Unexpected ClickOutput in foreign town menu".to_owned(),
                        ));
                        return;
                    }
                }
            }
        }
        let ms = MouseState(pos);
        state.town_world_mut().insert(ms);
        self.left_click_dispatcher.dispatch(state.town_world());
    }
    fn right_click(&mut self, state: &mut Game, _pos: Vector) {
        let town_world = state.town_world();
        // Right click cancels grabbed item (take removes from option)
        let mut ui_state = town_world.fetch_mut::<UiState>();
        ui_state.take_grabbed_item();
    }

    fn render_foreign_town_extras(
        &mut self,
        sprites: &mut Sprites,
        window: &mut DisplayArea,
        area: &Rectangle,
        now: NaiveDateTime,
        mouse_pos: Option<Vector>,
    ) {
        let mut table = vec![];
        table.push(TableRow::InteractiveArea(&mut self.foreign_town_menu));

        draw_table(
            window,
            sprites,
            &mut table,
            area,
            &mut self.text_provider,
            40.0,
            Z_UI_MENU,
            now,
            TableVerticalAlignment::Top,
            mouse_pos,
        );
    }

    pub fn new_story_state(
        &mut self,
        state: &mut Game,
        msg: &crate::game::dialogue::NewStoryState,
    ) {
        // FIXME: redundant with the same call also in dialogue
        state.set_story_state(msg.new_story_state);
        DefaultShop::reload(state.town_world_mut());
    }
    pub fn signal(&mut self, state: &mut Game, e: &Signal) {
        match e {
            Signal::ResourcesUpdated => {
                self.bank_component
                    .update(
                        &state
                            .town_world()
                            .fetch::<TownResources>()
                            .non_zero_resources(),
                    )
                    .nuts_check();
            }
            _ => {}
        }
    }
}
