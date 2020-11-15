use crate::gui::{
    gui_components::*, input::UiView, shapes::PadlShapeIndex, sprites::*, ui_state::Now, utils::*,
};
use crate::prelude::*;
use paddle::WebGLCanvas;
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
    type Error = PadlError;
    type State = Game;

    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut WebGLCanvas,
        _timestamp: f64,
    ) -> Result<(), Self::Error> {
        self.text_provider.reset();
        let inner_area = state.inner_menu_area();
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui.draw(
            window,
            sprites,
            &mut self.text_provider,
            now,
            &inner_area,
            state.mouse.pos(),
        )?;
        self.text_provider.finish_draw();
        Ok(())
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) -> Result<(), Self::Error> {
        let result = match self.ui.click(pos.into())? {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            nuts::publish(event);
        }
        Ok(())
    }
}
