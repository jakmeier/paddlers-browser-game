use crate::net::NetMsg;
use crate::prelude::*;
use crate::resolution::{MAIN_AREA_H, MAIN_AREA_W};
use crate::{
    game::toplevel::Signal,
    gui::{menu::LEAVES_BORDER_W, utils::colors::LIGHT_BLUE},
};
use div::doc;
use mogwai::prelude::*;
use paddle::{DisplayArea, FrameHandle};
use paddle::{JsError, NutsCheck};
use paddlers_shared_lib::prelude::VisitReportKey;
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
    fn letter_text(&self, id: usize) -> &'static str {
        match id as usize % 5 {
            0 => "Thank you, was a very enjoyable visit.",
            1 => "Cheers!",
            2 => "Thanks for showing me your town.",
            3 => "See you again soon.",
            4 => "A lovely place you have there.",
            _ => unreachable!(),
        }
    }
    fn number_of_reports(&self) -> usize {
        self.reports.len()
    }
    pub fn network_message(&mut self, _state: &mut Game, msg: &NetMsg) {
        match msg {
            NetMsg::Reports(data) => {
                for r in &data.village.reports {
                    let id = r.id.parse().unwrap();
                    self.add_report(Report {
                        id: VisitReportKey(id),
                        karma: r.karma,
                        feathers: r.resources.feathers,
                        logs: r.resources.logs,
                        sticks: r.resources.sticks,
                        text: self.letter_text(id as usize),
                    })
                    .nuts_check();
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

impl Frame for ReportFrame {
    type State = Game;
    const WIDTH: u32 = MAIN_AREA_W;
    const HEIGHT: u32 = MAIN_AREA_H;

    fn draw(&mut self, _state: &mut Self::State, window: &mut DisplayArea, _timestamp: f64) {
        window.fill(LIGHT_BLUE);
    }
    fn enter(&mut self, _state: &mut Self::State) {
        self.pane.show().nuts_check();
    }
    fn leave(&mut self, _state: &mut Self::State) {
        self.pane.hide().nuts_check();
    }
}
