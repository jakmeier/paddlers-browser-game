use crate::{gui::gui_components::mogwai_res_node, net::graphql::PlayerQuest};
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::*;
#[derive(Clone, Debug)]
pub struct ResourceReward {
    t: ResourceType,
    amount: i64,
}

impl ResourceReward {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        let r = &quest.rewards;
        if r.feathers > 0 {
            out.push(Self {
                t: ResourceType::Feathers,
                amount: r.feathers,
            });
        }
        if r.sticks > 0 {
            out.push(Self {
                t: ResourceType::Sticks,
                amount: r.sticks,
            });
        }
        if r.logs > 0 {
            out.push(Self {
                t: ResourceType::Logs,
                amount: r.logs,
            });
        }
        out
    }
    pub fn view_builder(self) -> ViewBuilder<HtmlElement> {
        Gizmo::from(self).view_builder()
    }
}

#[derive(Clone)]
pub enum RewardIn {}

impl Component for ResourceReward {
    type ModelMsg = RewardIn;
    type ViewMsg = ();
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        _msg: &RewardIn,
        _tx_view: &Transmitter<()>,
        _subscriber: &Subscriber<RewardIn>,
    ) {
    }

    #[allow(unused_braces)]
    fn view(&self, _tx: &Transmitter<RewardIn>, _rx: &Receiver<()>) -> ViewBuilder<HtmlElement> {
        builder!(
            <div class="reward">
                { mogwai_res_node(self.amount, self.t) }
            </div>
        )
    }
}
