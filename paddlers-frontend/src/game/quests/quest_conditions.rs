use crate::{
    game::town::Town,
    gui::sprites::{SingleSprite, SpriteIndex, Sprites, WithSprite},
};
use crate::{game::town_resources::TownResources, net::graphql::PlayerQuest};
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::*;

#[derive(Clone)]
pub struct KarmaCondition {
    amount: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}
#[derive(Clone)]
pub struct ResourceCondition {
    t: ResourceType,
    amount: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}

#[derive(Clone)]
pub struct BuildingCondition {
    t: BuildingType,
    amount: i64,
    cached_current: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}
#[derive(Clone)]
pub struct WorkerCondition {
    t: TaskType,
    amount: i64,
    cached_current: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}

impl KarmaCondition {
    pub fn new(karma_goal: i64, karma_now: i64) -> Self {
        let component = QuestConditionComponent::new(
            SpriteIndex::Simple(SingleSprite::Karma),
            karma_goal,
            karma_now,
        );
        Self {
            amount: karma_goal,
            gizmo: Gizmo::from(component),
        }
    }
    pub fn view_builder(&self) -> ViewBuilder<HtmlElement> {
        self.gizmo.view_builder()
    }
    pub fn update_karma(&self, karma: i64) {
        self.gizmo.send(&NewCurrentValue(karma));
    }
}

impl ResourceCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, bank: &TownResources) -> Vec<Self> {
        let mut out = vec![];
        let rcs = &quest.conditions.resources;

        if let Some(c) = Self::new(ResourceType::Feathers, rcs.feathers, bank) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Sticks, rcs.sticks, bank) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Logs, rcs.logs, bank) {
            out.push(c);
        }
        out
    }
    fn new(t: ResourceType, amount: i64, bank: &TownResources) -> Option<Self> {
        if amount <= 0 {
            return None;
        }
        let current = bank.read(t);
        let component = QuestConditionComponent::new(t.sprite().default(), amount, current);
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
}

impl BuildingCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, town: &Town) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.buildings {
            let t: BuildingType = (&c.building_type).into();
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
}

impl WorkerCondition {
    pub fn from_quest_ref(quest: &PlayerQuest, town: &Town) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.workers {
            let t: TaskType = (&c.task_type).into();
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
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub(super) enum QuestConditionViewUpdate {
    UpdateProgress(i64),
    MarkComplete,
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
            <div class={( if current < self.goal { "condition condition-in-progress".to_string() } else  { "condition condition-met".to_owned() } , rx.branch_map(css_class))}>
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
        if self.goal <= msg.0 {
            tx_view.send(&QuestConditionViewUpdate::UpdateProgress(self.goal));
            tx_view.send(&QuestConditionViewUpdate::MarkComplete);
        } else {
            tx_view.send(&QuestConditionViewUpdate::UpdateProgress(msg.0));
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

fn css_class(msg: &QuestConditionViewUpdate) -> String {
    if let QuestConditionViewUpdate::MarkComplete = msg {
        "condition condition-met".to_owned()
    } else {
        "condition condition-in-progress".to_owned()
    }
}
