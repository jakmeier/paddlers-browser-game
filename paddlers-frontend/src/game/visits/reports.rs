use crate::gui::sprites::*;
use crate::gui::utils::colors::LIGHT_BLUE;
use crate::gui::z::*;
use crate::net::NetMsg;
use crate::prelude::*;
use core::marker::PhantomData;
use div::doc;
use paddle::quicksilver_compat::{Col, Rectangle, Transform};
use paddle::WebGLCanvas;
use paddlers_shared_lib::prelude::VisitReportKey;
use specs::prelude::*;
use web_sys::{Element, Node};

pub(crate) struct ReportFrame<'a, 'b> {
    pane: div::PaneHandle,
    table: Node,
    _phantom: PhantomData<(&'a (), &'b ())>,
}

struct Report {
    id: VisitReportKey,
    karma: i64,
    feathers: i64,
    sticks: i64,
    logs: i64,
}

impl<'a, 'b> ReportFrame<'a, 'b> {
    pub fn new(area: Rectangle, resolution: ScreenResolution) -> PadlResult<Self> {
        let right_padding = resolution.leaves_border_w() * 0.75;
        let pane = div::new_pane(
            area.x() as u32,
            area.y() as u32,
            (area.width() - right_padding) as u32,
            area.height() as u32,
            r#"<section class="letters"></section>"#,
        )?;
        pane.hide()?;
        let node = pane.first_inner_node()?;

        let title = doc()?.create_element("h2").unwrap();
        title.set_text_content(Some("Mailbox"));
        node.append_child(&title)?;

        Ok(ReportFrame {
            pane,
            table: node,
            _phantom: Default::default(),
        })
    }
    fn add_report(&mut self, report: Report, sprites: &Sprites) -> PadlResult<()> {
        let letter_node = doc()?.create_element("div")?;
        letter_node.set_attribute("class", "letter")?;

        let text_node = doc()?.create_element("p")?;
        text_node.set_text_content(Some(&self.letter_text(&report)));
        letter_node.append_child(&text_node)?;

        if report.karma > 0 {
            letter_node.append_child(&self.new_res_node(
                report.karma,
                SingleSprite::Karma,
                sprites,
            ))?;
        }
        if report.feathers > 0 {
            letter_node.append_child(&self.new_res_node(
                report.feathers,
                SingleSprite::Feathers,
                sprites,
            ))?;
        }
        if report.sticks > 0 {
            letter_node.append_child(&self.new_res_node(
                report.sticks,
                SingleSprite::Sticks,
                sprites,
            ))?;
        }
        if report.logs > 0 {
            letter_node.append_child(&self.new_res_node(
                report.logs,
                SingleSprite::Logs,
                sprites,
            ))?;
        }

        let button_node = doc()?.create_element("div")?;
        button_node.set_attribute("class", "letter-button")?;
        button_node.set_text_content(Some(&"Collect"));
        self.add_listener(&button_node, report, letter_node.clone());

        letter_node.append_child(&button_node)?;

        self.table.append_child(&letter_node)?;
        Ok(())
    }
    #[allow(unused_variables)] //TODO
    fn add_listener(&self, button_node: &Element, report: Report, parent: Element) {
        let _table_ref = self.table.clone();

        // TODO XXX TODO
        // TODO XXX TODO
        // TODO XXX TODO

        // let _handle = button_node.add_event_listener::<event::ClickEvent, _>(move |_| {
        //     let _node = table_ref.remove_child(&parent).expect("Letter not found");
        //     let msg = ReportCollect {
        //         reports: vec![report.id],
        //     };
        //     if let Err(e) = RestApiState::get().http_collect_reward(msg) {
        //         println!("Failed to send API call {}", e);
        //     }
        //     crate::net::request_resource_update();
        //     // remove event listener?
        //     // TODO: Update notifications (now done for every left click)
        // });
    }
    fn new_res_node(&mut self, n: i64, s: SingleSprite, sprites: &Sprites) -> Element {
        let node = doc().unwrap().create_element("div").unwrap();
        node.set_attribute("class", "letter-res").unwrap();
        let num_node = doc().unwrap().create_element("div").unwrap();
        num_node.set_text_content(Some(&n.to_string()));
        let img = sprites.new_image_node(SpriteIndex::Simple(s));

        node.append_child(&num_node).unwrap();
        node.append_child(&Node::from(img)).unwrap();
        node
    }
    fn letter_text(&self, report: &Report) -> &'static str {
        match report.id.0 as usize % 5 {
            0 => "Thank you, was a very enjoyable visit.",
            1 => "Cheers!",
            2 => "Thanks for showing me your town.",
            3 => "See you again soon.",
            4 => "A lovely place you have there.",
            _ => unreachable!(),
        }
    }
    fn number_or_reports(&self) -> usize {
        self.table.child_nodes().length() as usize - 1 // -1 for title
    }
    pub fn network_message(&mut self, state: &mut Game, msg: &NetMsg) -> Result<(), PadlError> {
        match msg {
            NetMsg::Reports(data) => {
                for r in &data.village.reports {
                    self.add_report(
                        Report {
                            id: VisitReportKey(r.id.parse().unwrap()),
                            karma: r.karma,
                            feathers: r.feathers,
                            logs: r.logs,
                            sticks: r.sticks,
                        },
                        &state.sprites,
                    )?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a, 'b> Frame for ReportFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game;
    type Graphics = WebGLCanvas;
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
