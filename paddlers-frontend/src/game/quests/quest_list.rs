use super::quest_component::QuestComponent;
use mogwai::prelude::*;

/// Parent component to hold all quests
pub(super) struct QuestList {
    quest_components: Vec<Gizmo<QuestComponent>>,
}

#[derive(Clone)]
pub(super) enum QuestListIn {
    NewQuestComponent(QuestComponent),
    Clear,
}

#[derive(Clone)]
pub(super) enum QuestListOut {
    PatchQuestList(Patch<View<HtmlElement>>),
}
impl QuestList {
    pub fn new() -> Self {
        Self {
            quest_components: vec![],
        }
    }
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
