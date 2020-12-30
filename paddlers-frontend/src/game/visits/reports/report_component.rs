use crate::{
    game::{level::SpriteIndex, status_effects::SingleSprite},
    gui::sprites::Sprites,
    net::game_master_api::RestApiState,
};

use super::{RemoveReport, Report};
use mogwai::prelude::*;
use paddlers_shared_lib::api::reports::ReportCollect;

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
            nodes.push(new_res_node(self.karma, SingleSprite::Karma));
        }
        if self.feathers > 0 {
            nodes.push(new_res_node(self.feathers, SingleSprite::Feathers));
        }
        if self.sticks > 0 {
            nodes.push(new_res_node(self.sticks, SingleSprite::Sticks));
        }
        if self.logs > 0 {
            nodes.push(new_res_node(self.logs, SingleSprite::Logs));
        }
        let builder = builder!(
            <div class="letter">
                <p> { self.text } </p>
                { nodes.get(0).cloned() }
                { nodes.get(1).cloned() }
                { nodes.get(2).cloned() }
                { nodes.get(3).cloned() }
                { nodes.get(4).cloned() }
                { nodes.get(5).cloned() }
                <div on:click=tx_event class="letter-button">
                    "Collect"
                </div>
            </div>
        );
        builder
    }
}

#[allow(unused_braces)]
fn new_res_node(n: i64, s: SingleSprite) -> ViewBuilder<HtmlElement> {
    let img = Sprites::new_image_node_builder(SpriteIndex::Simple(s));
    builder!(
        <div class="letter-res">
            <div> { n.to_string() } </div>
            { img }
        </div>
    )
}
