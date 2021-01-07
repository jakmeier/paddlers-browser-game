use super::Game;
use crate::{
    net::{graphql::PlayerQuest, NetMsg},
    prelude::TextDb,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use mogwai::prelude::*;
use paddle::{Frame, FrameHandle};

/// A Mogwai component ot display a single quest
mod quest_component;
/// Conditions to meet to finish a quest.
mod quest_conditions;
/// A Mogwai component ot display a list of quests
mod quest_list;

use quest_component::*;
use quest_list::*;

pub(crate) struct QuestsFrame {
    /// For communication with spawned view
    quests_gizmo: Gizmo<QuestList>,
    /// For keeping component alive
    quests_view: View<HtmlElement>,
}

struct NewParent(HtmlElement);

impl Frame for QuestsFrame {
    type State = Game;
    const WIDTH: u32 = MAIN_AREA_W;
    const HEIGHT: u32 = MAIN_AREA_H;
}
impl QuestsFrame {
    pub fn new() -> Self {
        let quest_list = QuestList::new();
        let quests_gizmo = Gizmo::from(quest_list);
        let quests_view = View::from(quests_gizmo.view_builder());

        Self {
            quests_view,
            quests_gizmo,
        }
    }
    pub fn init_listeners(frame_handle: FrameHandle<Self>) {
        frame_handle.listen(Self::network_message);
        frame_handle.listen(Self::attach_to_parent);
        let div = frame_handle.div();
        paddle::share(NewParent(div.parent_element().unwrap()))
    }

    fn network_message(&mut self, state: &mut Game, msg: &NetMsg) {
        match msg {
            NetMsg::Quests(data) => {
                self.reset_quests();
                for quest in data {
                    self.add_quest(quest, &state.locale);
                }
            }
            _ => {}
        }
    }
    fn attach_to_parent(&mut self, _state: &mut Game, node: &NewParent) {
        node.0.append_child(&self.quests_view.dom_ref()).unwrap();
    }
    fn reset_quests(&mut self) {
        self.quests_gizmo.send(&QuestListIn::Clear);
    }
    fn add_quest(&mut self, quest: &PlayerQuest, locale: &TextDb) {
        self.quests_gizmo
            .send(&QuestListIn::NewQuestComponent(QuestComponent::new(
                quest, locale,
            )))
    }
}
