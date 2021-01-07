use crate::{
    net::{game_master_api::RestApiState, graphql::PlayerQuest},
    prelude::TextDb,
};

use super::{quest_conditions::*, quest_rewards::ResourceReward, QuestUiTexts};
use mogwai::prelude::*;
use paddlers_shared_lib::{
    api::quests::QuestCollect,
    prelude::{QuestKey, ResourceType},
};

#[derive(Clone)]
/// A Mogwai component ot display a single quest
pub(super) struct QuestComponent {
    id: QuestKey,
    title: String,
    text: String,
    // conditions
    building_conditions: Vec<BuildingCondition>,
    worker_conditions: Vec<WorkerCondition>,
    resource_conditions: Vec<ResourceCondition>,
    karma_condition: Option<i64>,
    // rewards
    resource_rewards: Vec<ResourceReward>,
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
            resource_rewards: ResourceReward::from_quest_ref(quest),
        }
    }
}

#[derive(Clone)]
pub(super) enum QuestIn {
    CollectMe,
    NewUiTexts(QuestUiTexts),
    ResourceUpdate(Vec<(ResourceType, i64)>),
}

impl Component for QuestComponent {
    type ModelMsg = QuestIn;
    type ViewMsg = QuestUiTexts;
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &QuestIn,
        tx_view: &Transmitter<Self::ViewMsg>,
        _subscriber: &Subscriber<QuestIn>,
    ) {
        match msg {
            QuestIn::NewUiTexts(uit) => {
                tx_view.send(uit);
            }
            QuestIn::CollectMe => {
                nuts::send_to::<RestApiState, _>(QuestCollect { quest: self.id });
            }
            QuestIn::ResourceUpdate(res) => {
                for child in &self.resource_conditions {
                    child.update_res(&res);
                }
            }
        }
    }

    #[allow(unused_braces)]
    fn view(
        &self,
        tx: &Transmitter<QuestIn>,
        rx: &Receiver<QuestUiTexts>,
    ) -> ViewBuilder<HtmlElement> {
        let tx_event = tx.contra_map(|_: &Event| QuestIn::CollectMe);
        builder!(
        <div class="quest">
            <h3> { &self.title } </h3>
            <p> { &self.text } </p>
            <div class="conditions">
                <div class="title"> { ("CONDITIONS", rx.branch_map(|uit| uit.conditions.clone())) }":" </div>
                { self.resource_conditions.get(0).map(ResourceCondition::view_builder) }
                { self.resource_conditions.get(1).map(ResourceCondition::view_builder) }
                { self.resource_conditions.get(2).map(ResourceCondition::view_builder) }
            </div>
            <div class="rewards">
                <div class="title"> { ("REWARDS", rx.branch_map(|uit| uit.rewards.clone())) }":" </div>
                { self.resource_rewards.get(0).cloned().map(ResourceReward::view_builder) }
                { self.resource_rewards.get(1).cloned().map(ResourceReward::view_builder) }
                { self.resource_rewards.get(2).cloned().map(ResourceReward::view_builder) }
            </div>
            <div on:click=tx_event class="button">
                "Collect"
            </div>
        </div>
        )
    }
}
