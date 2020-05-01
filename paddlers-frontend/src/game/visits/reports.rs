use crate::gui::sprites::*;
use crate::gui::ui_state::UiState;
use crate::gui::utils::colors::LIGHT_BLUE;
use crate::gui::z::*;
use crate::init::quicksilver_integration::Signal;
use crate::net::NetMsg;
use crate::prelude::*;
use crate::view::*;
use core::marker::PhantomData;
use paddlers_shared_lib::prelude::VisitReportKey;
use quicksilver::prelude::{Col, Rectangle, Transform, Window};
use specs::prelude::*;
use std::collections::VecDeque;
use stdweb::web::*;

pub(crate) struct ReportFrame<'a, 'b> {
    // reports: VecDeque<Report>,
    pane: panes::PaneHandle,
    table: Node,
    _phantom: PhantomData<(&'a (), &'b ())>,
}

struct Report {
    id: VisitReportKey,
    karma: i64,
    feathers: i64,
    // TODO: resources
}

impl<'a, 'b> ReportFrame<'a, 'b> {
    pub fn new(area: Rectangle) -> PadlResult<Self> {
        let pane = panes::new_pane(
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
            r#"<section class="letters"></section>"#,
        )?;
        let node = pane.first_inner_node()?;

        Ok(ReportFrame {
            // reports: VecDeque::new(),
            pane,
            table: node,
            _phantom: Default::default(),
        })
    }
    fn add_report(&mut self, report: Report, sprites: &Sprites) {
        let letter_node = document().create_element("div").unwrap();
        letter_node.set_attribute("class", "letter").unwrap();
        let karma_node = self.new_res_node(report.karma, SingleSprite::Karma, sprites);
        let feathers_node = self.new_res_node(report.feathers, SingleSprite::Feathers, sprites);
        letter_node.append_child(&karma_node);
        letter_node.append_child(&feathers_node);
        self.table.append_child(&letter_node);
    }
    fn new_res_node(&mut self, n: i64, s: SingleSprite, sprites: &Sprites) -> Element {
        let node = document().create_element("div").unwrap();
        node.set_attribute("class", "letter-res").unwrap();
        let num_node = document().create_element("div").unwrap();
        num_node.set_text_content(&n.to_string());
        let img = sprites.new_image_node(SpriteIndex::Simple(s));

        node.append_child(&num_node);
        node.append_child(&img);
        node
    }
}

impl<'a, 'b> Frame for ReportFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    type Signal = Signal;
    fn event(&mut self, state: &mut Self::State, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            PadlEvent::Network(NetMsg::Reports(data)) => {
                for r in &data.village.reports {
                    self.add_report(
                        Report {
                            id: VisitReportKey(r.id.parse().unwrap()),
                            karma: r.karma,
                            feathers: r.feathers,
                        },
                        &state.sprites,
                    )
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
        window.draw_ex(&main_area, Col(LIGHT_BLUE), Transform::IDENTITY, Z_TEXTURE);
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
