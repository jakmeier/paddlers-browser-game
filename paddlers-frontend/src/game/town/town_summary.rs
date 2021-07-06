//! A view that display a summary of all available resources, like forest size or nests
//!

use crate::gui::utils::colors::DARK_BLUE;
use crate::gui::z::*;
use crate::prelude::*;
use crate::{
    game::Game,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use paddle::NutsCheck;
use paddle::{Frame, Rectangle, Transform};

pub(crate) struct TownSummaryFrame {
    pane: div::DivHandle,
}

impl TownSummaryFrame {
    pub fn new() -> PadlResult<Self> {
        let area = Self::area();
        let pane = div::new_styled(
            area.x() as i32,
            area.y() as i32,
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

        Ok(TownSummaryFrame { pane })
    }
}

impl Frame for TownSummaryFrame {
    type State = Game;
    const WIDTH: u32 = crate::resolution::MAIN_AREA_W;
    const HEIGHT: u32 = crate::resolution::MAIN_AREA_H;
    fn draw(
        &mut self,
        _state: &mut Self::State,
        window: &mut paddle::DisplayArea,
        _timestamp: f64,
    ) {
        let main_area = Rectangle::new_sized((MAIN_AREA_W, MAIN_AREA_H));
        window.draw_ex(
            &main_area.padded(50.0),
            &DARK_BLUE,
            Transform::IDENTITY,
            Z_TEXTURE,
        );
    }
    fn enter(&mut self, _state: &mut Self::State) {
        self.pane.show().nuts_check();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.pane.hide().nuts_check();
    }
}
