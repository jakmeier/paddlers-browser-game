use crate::gui::{
    gui_components::*, input::UiView, shapes::PadlShapeIndex, sprites::*, ui_state::Now, utils::*,
};
use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use crate::view::*;
use core::marker::PhantomData;
use quicksilver::prelude::Window;
use specs::WorldExt;

pub(crate) struct VisitorMenuFrame<'a, 'b> {
    ui: UiBox,
    text_provider: TableTextProvider,
    _phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> VisitorMenuFrame<'a, 'b> {
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
            _phantom: Default::default(),
        }
    }
}

impl<'a, 'b> Frame for VisitorMenuFrame<'a, 'b> {
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
        let (sprites, now) = (&mut state.sprites, state.world.read_resource::<Now>().0);
        self.ui
            .draw(window, sprites, &mut self.text_provider, now, &inner_area)?;
        self.text_provider.finish_draw();
        Ok(())
    }
    fn left_click(
        &mut self,
        state: &mut Self::State,
        pos: (i32, i32),
        _signals: &mut ExperimentalSignalChannel,
    ) -> Result<(), Self::Error> {
        let result = match self.ui.click(pos.into())? {
            Some((ClickOutput::Event(event), _)) => Ok(Some(event)),
            _ => Ok(None),
        };
        if let Some(event) = state.check(result).flatten() {
            state
                .event_pool
                .send(event)
                .expect("Event pool send failed");
        }
        Ok(())
    }
}
