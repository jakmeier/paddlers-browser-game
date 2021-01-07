use crate::gui::sprites::{SpriteIndex, Sprites, WithSprite};
use crate::net::graphql::PlayerQuest;
use mogwai::prelude::*;
use paddlers_shared_lib::prelude::*;

#[derive(Clone)]
pub struct ResourceCondition {
    t: ResourceType,
    amount: i64,
    gizmo: Gizmo<QuestConditionComponent>,
}

#[derive(Clone, Debug)]
pub struct BuildingCondition {
    t: BuildingType,
    amount: i64,
}
#[derive(Clone, Debug)]
pub struct WorkerCondition {
    t: TaskType,
    amount: i64,
}

impl ResourceCondition {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        let rcs = &quest.conditions.resources;

        if let Some(c) = Self::new(ResourceType::Feathers, rcs.feathers) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Sticks, rcs.sticks) {
            out.push(c);
        }
        if let Some(c) = Self::new(ResourceType::Logs, rcs.logs) {
            out.push(c);
        }
        out
    }
    fn new(t: ResourceType, amount: i64) -> Option<Self> {
        if amount <= 0 {
            return None;
        }
        let component = QuestConditionComponent::new(t.sprite().default(), amount);
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
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.buildings {
            out.push(BuildingCondition {
                t: (&c.building_type).into(),
                amount: c.amount,
            })
        }
        out
    }
}

impl WorkerCondition {
    pub fn from_quest_ref(quest: &PlayerQuest) -> Vec<Self> {
        let mut out = vec![];
        for c in &quest.conditions.workers {
            out.push(WorkerCondition {
                t: (&c.task_type).into(),
                amount: c.amount,
            })
        }
        out
    }
}

#[derive(Clone, Debug)]
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
    pub fn new(sprite: SpriteIndex, goal: i64) -> Self {
        Self {
            sprite,
            goal,
            current: 0,
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
        builder!(
            <div class={("condition condition-in-progress".to_string(), rx.branch_map(css_class))}>
                <div> { (self.current.to_string(), rx.branch_filter_map(filter_progress_update)) } "/" {self.goal.to_string()} </div>
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
