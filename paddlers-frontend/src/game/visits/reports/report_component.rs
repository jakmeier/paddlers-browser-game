use crate::{
    gui::{gui_components::*, sprites::Sprites},
    net::game_master_api::RestApiState,
};

use super::{RemoveReport, Report};
use mogwai::prelude::*;
use paddlers_shared_lib::{api::reports::ReportCollect, prelude::ResourceType};

#[derive(Clone)]
pub enum ReportIn {
    CollectMe,
}

impl Report {
    fn collect_me(&mut self) {
        nuts::send_to::<RestApiState, _>(ReportCollect {
            reports: vec![self.id],
        });
        paddle::share(RemoveReport(self.id));
        crate::net::request_resource_update();
    }
}

impl Component for Report {
    type ModelMsg = ReportIn;
    type ViewMsg = ();
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &ReportIn,
        _tx_view: &Transmitter<()>,
        _subscriber: &Subscriber<ReportIn>,
    ) {
        match msg {
            ReportIn::CollectMe => {
                self.collect_me();
            }
        }
    }

    #[allow(unused_braces)]
    fn view(&self, tx: &Transmitter<ReportIn>, _rx: &Receiver<()>) -> ViewBuilder<HtmlElement> {
        let tx_event = tx.contra_map(|_: &Event| ReportIn::CollectMe);

        let mut nodes: Vec<ViewBuilder<HtmlElement>> = vec![];
        if self.karma > 0 {
            nodes.push(mogwai_karma_res_node(self.karma));
        }
        if self.feathers > 0 {
            nodes.push(mogwai_res_node(self.feathers, ResourceType::Feathers));
        }
        if self.sticks > 0 {
            nodes.push(mogwai_res_node(self.sticks, ResourceType::Sticks));
        }
        if self.logs > 0 {
            nodes.push(mogwai_res_node(self.logs, ResourceType::Logs));
        }

        let img = self.sender_image.map(Sprites::new_image_node_builder);

        let builder = builder!(
            <div class="letter">
                <div class="sender"> { img } </div>
                <p> { self.text } </p>
                { nodes.get(0).cloned() }
                { nodes.get(1).cloned() }
                { nodes.get(2).cloned() }
                { nodes.get(3).cloned() }
                { nodes.get(4).cloned() }
                { nodes.get(5).cloned() }
                <div on:click=tx_event class="button">
                    "Collect"
                </div>
            </div>
        );
        builder
    }
}
