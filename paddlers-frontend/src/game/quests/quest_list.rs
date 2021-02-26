use crate::game::player_info::PlayerState;

use super::{
    quest_component::{QuestComponent, QuestIn},
    QuestUiTexts,
};
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::{BuildingType, ResourceType, TaskType};

/// Parent component to hold all quests
pub(super) struct QuestList {
    quest_components: Vec<Gizmo<QuestComponent>>,
}

#[derive(Clone)]
pub(super) enum QuestListIn {
    NewLocale(QuestUiTexts),
    NewQuestComponent(QuestComponent),
    Clear,
    ResourceUpdate(Vec<(ResourceType, i64)>),
    BuildingChange(BuildingType, i64),
    PlayerState(PlayerState),
    WorkerChange(TaskType, i64),
}

#[derive(Clone)]
pub(super) enum QuestListOut {
    PatchQuestList(Patch<View<HtmlElement>>),
    NewTitle(String),
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
                tx.send(&QuestListOut::PatchQuestList(Patch::RemoveAll));
            }
            QuestListIn::NewLocale(ui_texts) => {
                tx.send(&QuestListOut::NewTitle(ui_texts.title.clone()));
                for q in &self.quest_components {
                    q.send(&QuestIn::NewUiTexts(ui_texts.clone()));
                }
            }
            QuestListIn::ResourceUpdate(res) => {
                for q in &self.quest_components {
                    q.send(&QuestIn::ResourceUpdate(res.clone()));
                }
            }
            QuestListIn::BuildingChange(b, n) => {
                for q in &self.quest_components {
                    q.send(&QuestIn::BuildingChange(*b, *n));
                }
            }
            QuestListIn::WorkerChange(task, n) => {
                for q in &self.quest_components {
                    q.send(&QuestIn::WorkerChange(*task, *n));
                }
            }
            QuestListIn::PlayerState(p) => {
                let karma = p.karma();
                let pop = p.pop();
                for q in &self.quest_components {
                    q.send(&QuestIn::Karma(karma));
                    q.send(&QuestIn::Population(pop));
                }
            }
        }
    }

    #[allow(unused_braces)]
    fn view(
        &self,
        _tx: &Transmitter<QuestListIn>,
        rx: &Receiver<QuestListOut>,
    ) -> ViewBuilder<HtmlElement> {
        builder! {
            <section class="quests">
                <h2>
                    { ("Quests", rx.branch_filter_map(filter_title)) }
                </h2>
                <div patch:children=rx.branch_filter_map(filter_path_quest)> </div>
            </section>
        }
    }
}

fn filter_title(msg: &QuestListOut) -> Option<String> {
    match msg {
        QuestListOut::NewTitle(title) => Some(title.clone()),
        _ => None,
    }
}
fn filter_path_quest(msg: &QuestListOut) -> Option<Patch<View<HtmlElement>>> {
    if let QuestListOut::PatchQuestList(patch) = msg {
        Some(patch.clone())
    } else {
        None
    }
}
