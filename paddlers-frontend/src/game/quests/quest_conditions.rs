use crate::{
    game::town::Town,
    gui::sprites::{SingleSprite, SpriteIndex, Sprites, WithSprite},
    prelude::ISpriteIndex,
};
use crate::{game::town_resources::TownResources, net::graphql::PlayerQuest};
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::*;

use super::quest_component::QuestIn;

#[derive(Clone)]
/// Tries to reach a fixed amount of *something*, receives updates of absolute values
pub(super) struct SimpleCondition {
    amount: i64,
    cached_current: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}
#[derive(Clone)]
/// Combines all resource conditions into one
pub(super) struct ResourceCondition {
    t: ResourceType,
    amount: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}

#[derive(Clone)]
pub(super) struct BuildingCondition {
    t: BuildingType,
    amount: i64,
    cached_current: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}
#[derive(Clone)]
pub(super) struct WorkerCondition {
    t: TaskType,
    amount: i64,
    cached_current: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}

impl SimpleCondition {
    pub fn new(karma_goal: i64, karma_now: i64, image: SingleSprite) -> Self {
        let component =
            QuestConditionComponent::new(SpriteIndex::Simple(image), karma_goal, karma_now);
        let gizmo = Gizmo::from(component);
        Self {
            cached_current: karma_now,
            amount: karma_goal,
            gizmo,
        }
    }
    pub fn view_builder(&self) -> ViewBuilder<HtmlElement> {
        self.gizmo.view_builder()
    }
    pub fn update(&mut self, karma: i64) {
        self.cached_current = karma;
        self.gizmo.send(&NewCurrentValue(karma));
    }
    pub fn subscriber(&mut self, sub: &Subscriber<QuestIn>) {
        sub.subscribe_map(&self.gizmo.recv, |msg| QuestIn::ChildMessage(msg.clone()));
    }
    pub fn is_complete(&self) -> bool {
        self.amount <= self.cached_current
    }
}

impl ResourceCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, bank: &TownResources) -> (Vec<Self>, usize) {
        let mut out = vec![];
        let mut completed = 0;
        let rcs = &quest.conditions.resources;

        if let Some(c) = Self::new(ResourceType::Feathers, rcs.feathers, bank, &mut completed) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Sticks, rcs.sticks, bank, &mut completed) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Logs, rcs.logs, bank, &mut completed) {
            out.push(c);
        }
        (out, completed)
    }
    fn new(
        t: ResourceType,
        amount: i64,
        bank: &TownResources,
        completed_counter: &mut usize,
    ) -> Option<Self> {
        if amount <= 0 {
            return None;
        }
        let current = bank.read(t);
        let component = QuestConditionComponent::new(t.sprite().default(), amount, current);
        if amount <= current {
            *completed_counter += 1;
        }
        Some(Self {
            t,
            amount,
            gizmo: Gizmo::from(component),
        })
    }
    pub fn view_builder(&self) -> ViewBuilder<HtmlElement> {
        self.gizmo.view_builder()
    }
    pub fn update_res(&self, res: &[(ResourceType, i64)]) {
        if let Some((_, n)) = res.iter().find(|(t, _)| *t == self.t) {
            self.gizmo.send(&NewCurrentValue(*n));
        }
    }
    pub fn subscriber(&mut self, sub: &Subscriber<QuestIn>) {
        sub.subscribe_map(&self.gizmo.recv, |msg| QuestIn::ChildMessage(msg.clone()));
    }
}

impl BuildingCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, town: &Town) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.buildings {
            let t: BuildingType = c.building_type;
            let amount = c.amount;
            let cached_current = town.count_building(t) as i64;
            let gizmo = Gizmo::new(QuestConditionComponent::new(
                t.sprite().default(),
                amount,
                cached_current,
            ));
            out.push(BuildingCondition {
                t,
                amount,
                gizmo,
                cached_current,
            })
        }
        out
    }
    pub fn view_builder(&self) -> ViewBuilder<HtmlElement> {
        self.gizmo.view_builder()
    }
    pub fn building_change(&mut self, bt: BuildingType, n: i64) {
        if self.t == bt {
            self.cached_current += n;
            self.gizmo.send(&NewCurrentValue(self.cached_current));
        }
    }
    pub fn is_complete(&self) -> bool {
        self.amount <= self.cached_current
    }
    pub fn subscriber(&mut self, sub: &Subscriber<QuestIn>) {
        sub.subscribe_map(&self.gizmo.recv, |msg| QuestIn::ChildMessage(msg.clone()));
    }
}

impl WorkerCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, town: &Town) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.workers {
            let t: TaskType = c.task_type;
            let amount = c.amount;
            let cached_current = town.count_workers(t) as i64;
            let gizmo = Gizmo::new(QuestConditionComponent::new(
                t.sprite().default(),
                amount,
                cached_current,
            ));
            out.push(WorkerCondition {
                t,
                amount,
                gizmo,
                cached_current,
            })
        }
        out
    }
    pub fn view_builder(&self) -> ViewBuilder<HtmlElement> {
        self.gizmo.view_builder()
    }
    pub fn worker_change(&mut self, t: TaskType, n: i64) {
        if self.t == t {
            self.cached_current += n;
            self.gizmo.send(&NewCurrentValue(self.cached_current));
        }
    }
    pub fn is_complete(&self) -> bool {
        self.amount <= self.cached_current
    }
    pub fn subscriber(&mut self, sub: &Subscriber<QuestIn>) {
        sub.subscribe_map(&self.gizmo.recv, |msg| QuestIn::ChildMessage(msg.clone()));
    }
}

#[derive(Clone)]
/// A Mogwai component that shows something like this:
///
///          X / Y [IMAGE]
///
/// Y and the IMAGE are fixed upon creation.
/// X can be updated. Its displayed value will be capped at Y (showing Y / Y if a value X greater than Y is set)
pub(super) struct QuestConditionComponent {
    sprite: SpriteIndex,
    goal: i64,
    current: i64,
}

#[derive(Clone, Debug)]
pub(super) struct NewCurrentValue(i64);

#[derive(Clone)]
pub(super) enum QuestConditionViewUpdate {
    UpdateProgress(i64),
    MarkComplete,
    MarkIncomplete,
}

impl QuestConditionComponent {
    pub fn new(sprite: SpriteIndex, goal: i64, current: i64) -> Self {
        Self {
            sprite,
            goal,
            current,
        }
    }
}

impl Component for QuestConditionComponent {
    type ModelMsg = NewCurrentValue;
    type ViewMsg = QuestConditionViewUpdate;
    type DomNode = HtmlElement;

    #[allow(unused_braces)]
    fn view(
        &self,
        _tx: &Transmitter<NewCurrentValue>,
        rx: &Receiver<QuestConditionViewUpdate>,
    ) -> ViewBuilder<HtmlElement> {
        let img = Sprites::new_image_node_builder(self.sprite);
        let current = self.current.min(self.goal);
        builder!(
            <div class={( if current < self.goal { "condition condition-in-progress".to_string() } else  { "condition condition-met".to_owned() } , rx.branch_filter_map(css_class))}>
                <div> { (current.to_string(), rx.branch_filter_map(filter_progress_update)) } "/" {self.goal.to_string()} </div>
                { img }
            </div>
        )
    }

    fn update(
        &mut self,
        msg: &NewCurrentValue,
        tx_view: &Transmitter<QuestConditionViewUpdate>,
        _subscriber: &Subscriber<NewCurrentValue>,
    ) {
        let before = self.goal <= self.current;
        self.current = msg.0;
        if self.goal <= self.current {
            tx_view.send(&QuestConditionViewUpdate::UpdateProgress(self.goal));
            if !before {
                tx_view.send(&QuestConditionViewUpdate::MarkComplete);
            }
        } else {
            tx_view.send(&QuestConditionViewUpdate::UpdateProgress(self.current));
            if before {
                tx_view.send(&QuestConditionViewUpdate::MarkIncomplete);
            }
        }
    }
}

fn filter_progress_update(msg: &QuestConditionViewUpdate) -> Option<String> {
    if let QuestConditionViewUpdate::UpdateProgress(n) = msg {
        Some(n.to_string())
    } else {
        None
    }
}

fn css_class(msg: &QuestConditionViewUpdate) -> Option<String> {
    if let QuestConditionViewUpdate::MarkComplete = msg {
        Some("condition condition-met".to_owned())
    } else if let QuestConditionViewUpdate::MarkIncomplete = msg {
        Some("condition condition-in-progress".to_owned())
    } else {
        None
    }
}
