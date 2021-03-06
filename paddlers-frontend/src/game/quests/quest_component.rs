use crate::{
    game::{player_info::PlayerState, town::Town, town_resources::TownResources},
    net::{game_master_api::RestApiState, graphql::PlayerQuest},
    prelude::TextDb,
};

use super::{quest_conditions::*, quest_rewards::ResourceReward, QuestUiTexts};
use mogwai::prelude::*;
use paddlers_shared_lib::{
    api::quests::QuestCollect,
    prelude::{BuildingType, QuestKey, ResourceType, TaskType},
    specification_types::SingleSprite,
};

#[derive(Clone)]
/// A Mogwai component ot display a single quest
pub(super) struct QuestComponent {
    id: QuestKey,
    title: String,
    text: String,
    init: bool,
    // conditions
    karma_condition: Option<SimpleCondition>,
    pop_condition: Option<SimpleCondition>,
    building_conditions: Vec<BuildingCondition>,
    worker_conditions: Vec<WorkerCondition>,
    resource_conditions: Vec<ResourceCondition>,
    // rewards
    resource_rewards: Vec<ResourceReward>,
    // progress tracking
    total_conditions: usize,
    completed_conditions: usize,
}

impl QuestComponent {
    pub(super) fn new(
        quest: &PlayerQuest,
        locale: &TextDb,
        town: &Town,
        bank: &TownResources,
        player: &PlayerState,
    ) -> Self {
        let id = quest.id.parse().unwrap();
        let key = &quest.key;
        let karma_condition = quest.conditions.karma.map(|karma_goal| {
            SimpleCondition::new(karma_goal, player.karma(), SingleSprite::Karma)
        });
        let pop_condition = quest
            .conditions
            .pop
            .map(|pop_goal| SimpleCondition::new(pop_goal, player.pop(), SingleSprite::Population));

        let building_conditions = BuildingCondition::from_quest_ref(quest, town);
        let worker_conditions = WorkerCondition::from_quest_ref(quest, town);
        let (resource_conditions, res_completed) = ResourceCondition::from_quest_ref(quest, bank);
        let resource_rewards = ResourceReward::from_quest_ref(quest);

        let buildings_completed = building_conditions
            .iter()
            .filter(|c| c.is_complete())
            .count();
        let worker_completed = worker_conditions.iter().filter(|c| c.is_complete()).count();

        let karma_completed = karma_condition
            .as_ref()
            .map(|c| c.is_complete())
            .unwrap_or(false);
        let karma_completed = if karma_completed { 1 } else { 0 };

        let pop_completed = pop_condition
            .as_ref()
            .map(|c| c.is_complete())
            .unwrap_or(false);
        let pop_completed = if pop_completed { 1 } else { 0 };

        let completed_conditions = res_completed
            + buildings_completed
            + worker_completed
            + karma_completed
            + pop_completed;

        let total_conditions = karma_condition.iter().count()
            + pop_condition.iter().count()
            + building_conditions.len()
            + worker_conditions.len()
            + resource_conditions.len();

        Self {
            id: QuestKey(id),
            title: locale.gettext(key).to_owned(),
            text: locale
                .gettext(&(key.to_owned() + "-description"))
                .to_owned(),
            karma_condition,
            pop_condition,
            building_conditions,
            worker_conditions,
            resource_conditions,
            resource_rewards,
            completed_conditions,
            total_conditions,
            init: false,
        }
    }
    fn subscribe_conditions(&mut self, sub: &Subscriber<QuestIn>) {
        for child in &mut self.building_conditions {
            child.subscriber(sub);
        }
        for child in &mut self.worker_conditions {
            child.subscriber(sub);
        }
        for child in &mut self.resource_conditions {
            child.subscriber(sub);
        }
        for child in &mut self.karma_condition {
            child.subscriber(sub);
        }
        for child in &mut self.pop_condition {
            child.subscriber(sub);
        }
    }
}

#[derive(Clone)]
pub(super) enum QuestIn {
    CollectMe,
    NewUiTexts(QuestUiTexts),
    ResourceUpdate(Vec<(ResourceType, i64)>),
    BuildingChange(BuildingType, i64),
    WorkerChange(TaskType, i64),
    Karma(i64),
    Population(i64),
    ChildMessage(QuestConditionViewUpdate),
}

#[derive(Clone)]
pub(super) enum QuestViewMessage {
    QuestUiTexts(QuestUiTexts),
    ReadyToCollect(bool),
}

impl Component for QuestComponent {
    type ModelMsg = QuestIn;
    type ViewMsg = QuestViewMessage;
    type DomNode = HtmlElement;

    fn update(
        &mut self,
        msg: &QuestIn,
        tx_view: &Transmitter<Self::ViewMsg>,
        subscriber: &Subscriber<QuestIn>,
    ) {
        if !self.init {
            self.subscribe_conditions(subscriber);
            self.init = true;
        }

        match msg {
            QuestIn::NewUiTexts(uit) => {
                tx_view.send(&QuestViewMessage::QuestUiTexts(uit.clone()));
            }
            QuestIn::CollectMe => {
                nuts::send_to::<RestApiState, _>(QuestCollect { quest: self.id });
            }
            QuestIn::ResourceUpdate(res) => {
                for child in &self.resource_conditions {
                    child.update_res(&res);
                }
            }
            QuestIn::BuildingChange(b, n) => {
                for child in &mut self.building_conditions {
                    child.building_change(*b, *n);
                }
            }
            QuestIn::WorkerChange(task, n) => {
                for child in &mut self.worker_conditions {
                    child.worker_change(*task, *n);
                }
            }
            QuestIn::Karma(karma) => {
                if let Some(child) = &mut self.karma_condition {
                    child.update(*karma);
                }
            }
            QuestIn::Population(pop) => {
                if let Some(child) = &mut self.pop_condition {
                    child.update(*pop);
                }
            }
            QuestIn::ChildMessage(child_msg) => {
                match child_msg {
                    QuestConditionViewUpdate::MarkComplete => {
                        self.completed_conditions += 1;
                    }
                    QuestConditionViewUpdate::MarkIncomplete => {
                        self.completed_conditions -= 1;
                    }
                    _ => {}
                }
                tx_view.send(&QuestViewMessage::ReadyToCollect(
                    self.completed_conditions == self.total_conditions,
                ));
            }
        }
    }

    #[allow(unused_braces)]
    fn view(
        &self,
        tx: &Transmitter<QuestIn>,
        rx: &Receiver<QuestViewMessage>,
    ) -> ViewBuilder<HtmlElement> {
        let tx_event = tx.contra_map(|_: &Event| QuestIn::CollectMe);

        let ready_now = self.completed_conditions == self.total_conditions;
        let ui_texts_rx = rx.branch_filter_map(ui_texts_filter);

        let visible_now = visibility(&ready_now);
        let visible_receiver = rx
            .branch_filter_map(completed_filter)
            .branch_map(visibility);

        // Note: Until I learn a better way to display vectors of nodes  in RSX,
        // I'll just assume a max number and use get() to optionally display each element.
        builder!(
        <div class="quest">
            <h3> { &self.title } </h3>
            <p> { &self.text } </p>
            <div class="conditions">
                <div class="title"> { ("CONDITIONS", ui_texts_rx.branch_map(|uit| uit.conditions.clone())) }":" </div>
                { self.karma_condition.as_ref().map(SimpleCondition::view_builder) }
                { self.pop_condition.as_ref().map(SimpleCondition::view_builder) }
                { self.building_conditions.get(0).map(BuildingCondition::view_builder) }
                { self.building_conditions.get(1).map(BuildingCondition::view_builder) }
                { self.building_conditions.get(2).map(BuildingCondition::view_builder) }
                { self.building_conditions.get(3).map(BuildingCondition::view_builder) }
                { self.building_conditions.get(4).map(BuildingCondition::view_builder) }
                { self.worker_conditions.get(0).map(WorkerCondition::view_builder) }
                { self.worker_conditions.get(1).map(WorkerCondition::view_builder) }
                { self.worker_conditions.get(2).map(WorkerCondition::view_builder) }
                { self.resource_conditions.get(0).map(ResourceCondition::view_builder) }
                { self.resource_conditions.get(1).map(ResourceCondition::view_builder) }
                { self.resource_conditions.get(2).map(ResourceCondition::view_builder) }
            </div>
            <div class="rewards">
                <div class="title"> { ("REWARDS", ui_texts_rx.branch_map(|uit| uit.rewards.clone())) }":" </div>
                { self.resource_rewards.get(0).cloned().map(ResourceReward::view_builder) }
                { self.resource_rewards.get(1).cloned().map(ResourceReward::view_builder) }
                { self.resource_rewards.get(2).cloned().map(ResourceReward::view_builder) }
            </div>
            <div on:click=tx_event class="button" style:visibility={(visible_now,visible_receiver)}>
                "Collect"
            </div>
        </div>
        )
    }
}

fn ui_texts_filter(msg: &QuestViewMessage) -> Option<QuestUiTexts> {
    if let QuestViewMessage::QuestUiTexts(uit) = msg {
        Some(uit.clone())
    } else {
        None
    }
}
fn completed_filter(msg: &QuestViewMessage) -> Option<bool> {
    if let QuestViewMessage::ReadyToCollect(ready) = msg {
        Some(*ready)
    } else {
        None
    }
}
fn visibility(show: &bool) -> String {
    if *show { "visible" } else { "hidden" }.to_string()
}
