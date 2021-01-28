use crate::{
    game::toplevel::Signal,
    gui::{menu::LEAVES_BORDER_W, utils::colors::LIGHT_BLUE},
};
use crate::{
    game::units::attackers::hobo_sprite_happy,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use crate::{gui::sprites::SingleSprite, prelude::*};
use crate::{
    gui::sprites::SpriteIndex,
    net::{graphql::ReportsResponseReport, state::current_village, NetMsg},
};
use div::doc;
use mogwai::prelude::*;
use paddle::{DisplayArea, FrameHandle};
use paddle::{JsError, NutsCheck};
use paddlers_shared_lib::prelude::{VillageKey, VisitReportKey};
use web_sys::Node;

mod report_component;
pub use report_component::*;

struct RemoveReport(VisitReportKey);

pub(crate) struct ReportFrame {
    pane: div::DivHandle,
    table_node: Node,
    reports: Vec<(VisitReportKey, View<HtmlElement>)>,
}

struct Report {
    id: VisitReportKey,
    text: &'static str,
    karma: i64,
    feathers: i64,
    sticks: i64,
    logs: i64,
    sender_image: Option<SpriteIndex>,
}

impl ReportFrame {
    pub fn new() -> PadlResult<Self> {
        let area = Self::area();
        let right_padding = LEAVES_BORDER_W * 0.75;
        let pane = div::new(
            area.x() as u32,
            area.y() as u32,
            (area.width() - right_padding) as u32,
            area.height() as u32,
            r#"<section class="letters"></section>"#,
        )?;
        pane.hide()?;
        let table_node = pane.first_inner_node()?;

        let title = doc()?.create_element("h2").unwrap();
        title.set_text_content(Some("Mailbox"));
        table_node.append_child(&title)?;

        Ok(ReportFrame {
            pane,
            table_node,
            reports: vec![],
        })
    }
    pub fn init_listeners(frame_handle: FrameHandle<Self>) {
        frame_handle.listen(ReportFrame::network_message);
        frame_handle.listen(ReportFrame::remove_report);
    }
    fn add_report(&mut self, report: Report) -> PadlResult<()> {
        let id = report.id;
        let gizmo = Gizmo::from(report);
        let view = View::from(gizmo.view_builder());

        let letter_node = view.dom_ref().clone();

        self.table_node.append_child(&letter_node)?;
        self.reports.push((id, view));
        Ok(())
    }
    fn number_of_reports(&self) -> usize {
        self.reports.len()
    }
    pub fn network_message(&mut self, _state: &mut Game, msg: &NetMsg) {
        match msg {
            NetMsg::Reports(data) => {
                for r in &data.village.reports {
                    let id = r.id.parse().unwrap();
                    let report = if let Some(sender) = &r.sender {
                        let unit_color = match &sender.color {
                            None => UnitColor::Yellow,
                            Some(col) => col.into(),
                        };
                        if VillageKey(sender.home.id) == current_village() {
                            Report::inhabitant_letter(id, r, Some(unit_color))
                        } else {
                            Report::visitor_letter(id, r, Some(unit_color))
                        }
                    } else {
                        Report::visitor_letter(id, r, None)
                    };
                    self.add_report(report).nuts_check();
                }
            }
            _ => {}
        }
    }
    fn remove_report(&mut self, _: &mut Game, msg: &RemoveReport) {
        if let Some(index) = self.reports.iter().position(|r| r.0 == msg.0) {
            let (_, view) = self.reports.swap_remove(index);
            self.table_node
                .remove_child(&view.dom_ref())
                .map_err(JsError::from_js_value)
                .map_err(PadlError::from)
                .nuts_check();
            paddle::share(Signal::NewReportCount(self.number_of_reports()));
        }
    }
}

impl Report {
    pub fn visitor_letter(
        id: i64,
        r: &ReportsResponseReport,
        sender_color: Option<UnitColor>,
    ) -> Self {
        let sender_image = sender_color.map(|color| SpriteIndex::Simple(hobo_sprite_happy(color)));
        Report {
            id: VisitReportKey(id),
            karma: r.karma,
            feathers: r.resources.feathers,
            logs: r.resources.logs,
            sticks: r.resources.sticks,
            text: visitor_letter_text(id as usize),
            sender_image,
        }
    }
    pub fn inhabitant_letter(
        id: i64,
        r: &ReportsResponseReport,
        sender_color: Option<UnitColor>,
    ) -> Self {
        let total_res_reward = r.resources.feathers + r.resources.logs + r.resources.sticks;
        let sender_image =
            sender_color.map(|_color| SpriteIndex::Simple(SingleSprite::SittingYellowDuck));
        Report {
            id: VisitReportKey(id),
            karma: r.karma,
            feathers: r.resources.feathers,
            logs: r.resources.logs,
            sticks: r.resources.sticks,
            text: inhabitant_letter_text(id as usize, total_res_reward > 0),
            sender_image,
        }
    }
}

impl Frame for ReportFrame {
    type State = Game;
    const WIDTH: u32 = MAIN_AREA_W;
    const HEIGHT: u32 = MAIN_AREA_H;

    fn draw(&mut self, _state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        window.fill(&LIGHT_BLUE);
    }
    fn enter(&mut self, _state: &mut Self::State) {
        self.pane.show().nuts_check();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.pane.hide().nuts_check();
    }
}

fn visitor_letter_text(id: usize) -> &'static str {
    match id as usize % 5 {
        0 => "Thank you, was a very enjoyable visit.",
        1 => "Cheers!",
        2 => "Thanks for showing me your town.",
        3 => "See you again soon.",
        4 => "A lovely place you have there.",
        _ => unreachable!(),
    }
}
fn inhabitant_letter_text(id: usize, has_reward: bool) -> &'static str {
    if has_reward {
        match id as usize % 5 {
            0 => "Please accept this present.",
            1 => "For you, my lord.",
            2 => "With sincerest gratitude.",
            3 => "I wish I could give more.",
            4 => "For you will know best how to use this.",
            _ => unreachable!(),
        }
    } else {
        match id as usize % 5 {
            0 => "Thank you for my home.",
            1 => "Praise upon you!",
            2 => "I wish I could spare you a sacrifice but I'm broke...",
            3 => "Cheers, mate.",
            4 => "😁",
            _ => unreachable!(),
        }
    }
}
