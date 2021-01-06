use super::Game;
use crate::{
    net::{graphql::PlayerQuest, NetMsg},
    prelude::TextDb,
    resolution::{MAIN_AREA_H, MAIN_AREA_W},
};
use mogwai::prelude::*;
use paddle::{Frame, FrameHandle};
use paddlers_shared_lib::prelude::QuestKey;

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
        let quest_list = QuestList {
            quest_components: vec![],
        };
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
        let id = quest.id.parse().unwrap();
        let key = &quest.key;
        self.quests_gizmo
            .send(&QuestListIn::NewQuestComponent(QuestComponent {
                id: QuestKey(id),
                title: locale.gettext(key).to_owned(),
                text: locale
                    .gettext(&(key.to_owned() + "-description"))
                    .to_owned(),
            }))
    }
}

// Parent component to hold all quests
struct QuestList {
    quest_components: Vec<Gizmo<QuestComponent>>,
}

#[derive(Clone)]
enum QuestListIn {
    NewQuestComponent(QuestComponent),
    Clear,
}

#[derive(Clone)]
enum QuestListOut {
    PatchQuestList(Patch<View<HtmlElement>>),
}

impl Component for QuestList {
    type ModelMsg = QuestListIn;
    type ViewMsg = QuestListOut;
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &QuestListIn,
        tx: &Transmitter<QuestListOut>,
        _sub: &Subscriber<QuestListIn>,
    ) {
        match msg {
            QuestListIn::NewQuestComponent(quest_component) => {
                let gizmo: Gizmo<QuestComponent> = Gizmo::from(quest_component.clone());

                let view: View<HtmlElement> = View::from(gizmo.view_builder());
                tx.send(&QuestListOut::PatchQuestList(Patch::PushBack {
                    value: view,
                }));
                self.quest_components.push(gizmo);
            }
            QuestListIn::Clear => {
                self.quest_components.clear();
            }
        }
    }

    fn view(
        &self,
        _tx: &Transmitter<QuestListIn>,
        rx: &Receiver<QuestListOut>,
    ) -> ViewBuilder<HtmlElement> {
        builder! {
            <section>
                <h2>"Duties"</h2>
                <div patch:children=rx.branch_map(|QuestListOut::PatchQuestList(patch)| patch.clone())></div>
            </section>
        }
    }
}

#[derive(Clone)]
struct QuestComponent {
    id: QuestKey,
    title: String,
    text: String,
    //TODO
    // all conditions
    // rewards
}

#[derive(Clone)]
pub enum QuestIn {
    CollectMe,
}

impl Component for QuestComponent {
    type ModelMsg = QuestIn;
    type ViewMsg = ();
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &QuestIn,
        _tx_view: &Transmitter<()>,
        _subscriber: &Subscriber<QuestIn>,
    ) {
        match msg {
            QuestIn::CollectMe => {
                // TODO: Send request to backend
                println!("Quest reward collecting not implemented");
            }
        }
    }

    #[allow(unused_braces)]
    fn view(&self, tx: &Transmitter<QuestIn>, _rx: &Receiver<()>) -> ViewBuilder<HtmlElement> {
        let tx_event = tx.contra_map(|_: &Event| QuestIn::CollectMe);
        builder!(
        <div class="quest">
            <h3> { &self.title } </h3>
            <p> { &self.text } </p>
            <div on:click=tx_event class="letter-button">
                "Collect"
            </div>
        </div>
        )
    }
}
