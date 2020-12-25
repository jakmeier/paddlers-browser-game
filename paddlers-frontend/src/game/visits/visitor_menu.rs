use crate::gui::{
    gui_components::*, input::UiView, shapes::PadlShapeIndex, sprites::*, ui_state::Now, utils::*,
};
use crate::prelude::*;
use paddle::DisplayArea;
use specs::WorldExt;

pub(crate) struct VisitorMenuFrame {
    ui: UiBox,
    text_provider: TableTextProvider,
}

impl VisitorMenuFrame {
    pub fn new() -> Self {
        let mut ui_box = UiBox::new(1, 5, 0.0, 10.0);
        let tabs = [
            (VisitorViewTab::Letters, SingleSprite::Letters),
            (VisitorViewTab::IncomingAttacks, SingleSprite::DuckShapes),
        ];
        for (view, img) in &tabs {
            let rend =
                RenderVariant::ImgWithHoverShape(SpriteSet::Simple(*img), PadlShapeIndex::Frame);
            ui_box.add(
                UiElement::new(GameEvent::SwitchToView(UiView::Visitors(*view)))
                    .with_render_variant(rend),
            );
        }

        VisitorMenuFrame {
            ui: ui_box,
            text_provider: TableTextProvider::new(),
        }
    }
}

impl Frame for VisitorMenuFrame {
    type State = Game;
    const WIDTH: u32 = crate::gui::menu::INNER_MENU_AREA_W as u32;
    const HEIGHT: u32 = crate::gui::menu::INNER_MENU_AREA_H as u32;

    fn draw(&mut self, state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        self.text_provider.reset();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui.draw(
            window,
            sprites,
            &mut self.text_provider,
            now,
            &Self::area(),
            state.mouse.pos(),
        );
        self.text_provider.finish_draw();
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        let result = match self.ui.click(pos.into()) {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            nuts::publish(event);
        }
    }
}
