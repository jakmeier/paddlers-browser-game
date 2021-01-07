use crate::{net::graphql::PlayerQuest, prelude::TextDb};

use super::quest_conditions::*;
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::QuestKey;

#[derive(Clone)]
/// A Mogwai component ot display a single quest
pub(super) struct QuestComponent {
    id: QuestKey,
    title: String,
    text: String,
    building_conditions: Vec<BuildingCondition>,
    worker_conditions: Vec<WorkerCondition>,
    resource_conditions: Vec<ResourceCondition>,
    karma_condition: Option<i64>,
    // TODO: rewards
}

impl QuestComponent {
    pub(super) fn new(quest: &PlayerQuest, locale: &TextDb) -> Self {
        let id = quest.id.parse().unwrap();
        let key = &quest.key;
        Self {
            id: QuestKey(id),
            title: locale.gettext(key).to_owned(),
            text: locale
                .gettext(&(key.to_owned() + "-description"))
                .to_owned(),
            building_conditions: BuildingCondition::from_quest_ref(quest),
            worker_conditions: WorkerCondition::from_quest_ref(quest),
            resource_conditions: ResourceCondition::from_quest_ref(quest),
            karma_condition: quest.conditions.karma,
        }
    }
}

#[derive(Clone)]
pub(super) enum QuestIn {
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
