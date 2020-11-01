//! A view that display a summary of all available resources, like forest size or nests
//!

use crate::game::Game;
use crate::gui::utils::colors::DARK_BLUE;
use crate::gui::z::*;
use crate::prelude::*;
use paddle::quicksilver_compat::{Col, Rectangle, Transform};
use paddle::Window as QuicksilverWindow;
use paddle::{Frame, JmrRectangle};
use specs::WorldExt;
use std::marker::PhantomData;

pub(crate) struct TownSummaryFrame<'a, 'b> {
    pane: div::PaneHandle,
    phantom: PhantomData<(&'a (), &'b ())>,
}

impl TownSummaryFrame<'_, '_> {
    pub fn new(area: &Rectangle) -> PadlResult<Self> {
        let pane = div::new_styled_pane(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="townsummary"><p onload="window.rusvelte.Test({target: this}, props: {})"></p></section>"#,
            &[""],
            &[("color", "white")],
        )?;
        // let node = pane.first_inner_node()?;
        // let jsCode = include_str!("dist/main.js");
        // let html = include_str!("Test.Svelte");
        // let new_node = Node::from_html(html).map_err(|_| "Syntax Error")?;
        // node.append_child(&new_node);
        // js!{

        //     new window.rusvelte.Test({target: @{node}, props: {}});

        //     // new Test({target: @{node}, props: {}});
        // };

        // // document.head.appendChild(script);
        pane.hide()?;

        Ok(TownSummaryFrame {
            pane,
            phantom: PhantomData,
        })
    }
}

impl<'a, 'b> Frame for TownSummaryFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game;
    type Graphics = QuicksilverWindow;

    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        let ui_state = state.world.read_resource::<ViewState>();
        let main_area = Rectangle::new(
            (0, 0),
            (
                ui_state.menu_box_area.x(),
                (window.project() * window.screen_size()).y,
            ),
        );
        std::mem::drop(ui_state);
        window.draw_ex(
            &main_area.padded(50.0),
            Col(DARK_BLUE),
            Transform::IDENTITY,
            Z_TEXTURE,
        );
        Ok(())
    }
    fn enter(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.pane.show()?;
        Ok(())
    }
    fn leave(&mut self, _state: &mut Self::State) -> Result<(), Self::Error> {
        self.pane.hide()?;
        Ok(())
    }
}
