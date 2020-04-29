use crate::prelude::*;
use specs::WorldExt;
use stdweb::web::*;
// use stdweb::unstable::TryInto;
use crate::game::Game;
use crate::gui::ui_state::UiState;
use crate::gui::utils::colors::DARK_BLUE;
use crate::gui::z::*;
use crate::init::quicksilver_integration::Signal;
use crate::net::NetMsg;
use crate::view::Frame;
use quicksilver::prelude::Window as QuicksilverWindow;
use quicksilver::prelude::{Col, Rectangle, Transform};
use std::marker::PhantomData;

pub(crate) struct LeaderboardFrame<'a, 'b> {
    pane: panes::PaneHandle,
    table: Node,
    phantom: PhantomData<(&'a (), &'b ())>,
}

impl LeaderboardFrame<'_, '_> {
    pub fn new(area: &Rectangle) -> PadlResult<Self> {
        let pane = panes::new_styled_pane(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="leaderboard"></section>"#,
            &[""],
            &[("color", "white")],
        )?;
        let node = pane.first_inner_node()?;

        // TODO Debug why this didn't work:

        // js! {
        //     console.log(@{node.as_ref()})
        // }

        // let el : HtmlElement = node.clone().try_into().map_err(
        //     |_| PadlError::dev_err(PadlErrorCode::InvalidDom("Not html"))
        // )?;

        // el.append_html(
        //     &format!(r#"<h3>{}</h3>
        //     <h3>{}</h3>
        //     <h3>{}</h3>
        //     "#,
        //     "#", "Name", "Karma")
        // ).expect("append html");

        // insert_h3(&node, "#");
        // insert_h3(&node, "Name");
        // insert_h3(&node, "Karma");

        pane.hide()?;

        Ok(LeaderboardFrame {
            pane,
            table: node,
            phantom: PhantomData,
        })
    }
    pub fn clear(&self) -> PadlResult<()> {
        self.table.remove_all_children();
        Ok(())
    }

    pub fn insert_row(&self, rank: usize, name: &str, karma: i64) -> PadlResult<()> {
        let node = document().create_element("div").unwrap();
        node.set_text_content(&rank.to_string());
        self.table.append_child(&node);

        let node = document().create_element("div").unwrap();
        node.set_text_content(name);
        self.table.append_child(&node);

        let node = document().create_element("div").unwrap();
        node.set_text_content(&karma.to_string());
        self.table.append_child(&node);

        Ok(())
    }
}

impl<'a, 'b> Frame for LeaderboardFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = QuicksilverWindow;
    type Event = PadlEvent;
    type Signal = Signal;
    fn event(&mut self, _state: &mut Self::State, e: &Self::Event) -> Result<(), Self::Error> {
        match e {
            PadlEvent::Network(NetMsg::Leaderboard(offset, list)) => {
                self.clear()?;
                for (i, (name, karma)) in list.into_iter().enumerate() {
                    self.insert_row(offset + i, &name, *karma)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn draw(
        &mut self,
        state: &mut Self::State,
        window: &mut Self::Graphics,
    ) -> Result<(), Self::Error> {
        let ui_state = state.world.read_resource::<UiState>();
        let main_area = Rectangle::new(
            (0, 0),
            (
                ui_state.menu_box_area.x(),
                (window.project() * window.screen_size()).y,
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

// fn insert_h3(node: &Node, text: &str) {
//     let inner = document().create_element("h3").unwrap();
//     inner.set_text_content(text);
//     node.append_child(&inner);
//     std::mem::drop(inner);
// }
