use crate::game::Game;
use crate::gui::utils::colors::DARK_BLUE;
use crate::gui::z::*;
use crate::net::NetMsg;
use crate::prelude::*;
use div::doc;
use paddle::quicksilver_compat::{Col, Rectangle, Transform};
use paddle::Frame;
use paddle::WebGLCanvas as QuicksilverWindow;
use specs::WorldExt;
use std::marker::PhantomData;
use web_sys::Node;

pub(crate) struct LeaderboardFrame<'a, 'b> {
    pane: div::PaneHandle,
    table: Node,
    phantom: PhantomData<(&'a (), &'b ())>,
}

impl LeaderboardFrame<'_, '_> {
    pub fn new(area: &Rectangle) -> PadlResult<Self> {
        let pane = div::new_styled_pane(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="leaderboard"></section>"#,
            &[""],
            &[("color", "white")],
        )?;
        let node = pane.first_inner_node()?;

        pane.hide()?;

        Ok(LeaderboardFrame {
            pane,
            table: node,
            phantom: PhantomData,
        })
    }
    pub fn clear(&self) -> PadlResult<()> {
        while let Some(child) = self.table.last_child() {
            self.table.remove_child(&child)?;
        }
        Ok(())
    }

    pub fn insert_row(&self, rank: usize, name: &str, karma: i64) -> PadlResult<()> {
        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&rank.to_string()));
        self.table.append_child(&node)?;

        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&name));
        self.table.append_child(&node)?;

        let node = doc()?.create_element("div")?;
        node.set_text_content(Some(&karma.to_string()));
        self.table.append_child(&node)?;

        Ok(())
    }

    pub fn network_message(&mut self, _state: &mut Game, msg: &NetMsg) -> Result<(), PadlError> {
        match msg {
            NetMsg::Leaderboard(offset, list) => {
                self.clear()?;
                for (i, (name, karma)) in list.into_iter().enumerate() {
                    self.insert_row(offset + i, &name, *karma)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a, 'b> Frame for LeaderboardFrame<'a, 'b> {
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
                (window.project() * window.browser_region().size()).y,
            ),
        );
        std::mem::drop(ui_state);
        window.draw_ex(&main_area, Col(DARK_BLUE), Transform::IDENTITY, Z_TEXTURE);
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
