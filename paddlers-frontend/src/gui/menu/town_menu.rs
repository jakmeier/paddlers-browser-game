use crate::gui::input::Grabbable;
use crate::prelude::*;
use crate::view::FloatingText;
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
    res_floats: [FloatingText;3],
    shop_floats: [FloatingText;3],
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
            res_floats: FloatingText::new_triplet()?,
            shop_floats: FloatingText::new_triplet()?,
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
        let inner_area = state.render_menu_box(window)?;
        let resolution = *state.world.read_resource::<ScreenResolution>();
        let resources_height = resolution.resources_h();
        let entity = self.selected_entity;
        
        let (resources_area, menu_area) = inner_area.cut_horizontal(resources_height);
        state.render_resources(window, &resources_area, &mut self.res_floats)?;
        render_town_menu(state, window, entity, &menu_area, &mut self.shop_floats, &mut self.res_floats)?;
        Ok(())
    }
    fn enter(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        for float in &mut self.res_floats {
            float.show()?;
        }
        for float in &mut self.shop_floats {
            float.show()?;
        }
        Ok(())
    }
    fn leave(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        for float in &mut self.res_floats {
            float.hide()?;
        }
        for float in &mut self.shop_floats {
            float.hide()?;
        }
        Ok(())
    }
    fn update(&mut self, state: &mut Self::State) -> Result<(),Self::Error> {
        //TODO
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32,i32)) -> Result<(),Self::Error> {
        let mut ms = state.world.write_resource::<MouseState>();
        *ms = MouseState(pos.into(), Some(MouseButton::Left));
        self.left_click_dispatcher.dispatch(&state.world);
        Ok(())
    }
}

fn render_town_menu(
    state: &mut Game<'_,'_>,
    window: &mut Window,
    entity: Option<Entity>,
    area: &Rectangle,
    floats: &mut [FloatingText;3],
    res_floats: &mut [FloatingText;3],
) -> PadlResult<()> {
    match entity {
        Some(id) => {
            state.render_entity_details(window, area, id, floats)?;
        },
        None => {
            state.render_default_shop(window, area, res_floats)?;
        },
    }
    Ok(())
}