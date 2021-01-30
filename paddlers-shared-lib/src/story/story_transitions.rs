//! Each player is in one StoryState, depending on the tutorial/story progression.
//!
//! The StoryState values are stored in the database per player and provided as PlayerInfo to the frontend.
//! Transitions are performed in the game-master when a StoryTrigger happens, following the FSM definied in `fn transition`.
//! In each transition, a set of StoryActions is also performed in the game-master and/or frontend.

use super::{story_action::StoryActionList, story_state::StoryState, story_trigger::StoryChoice};
use crate::story::story_action::StoryAction;
use crate::story::story_trigger::StoryTrigger;
use crate::{const_list::ConstList, prelude::BuildingType};
use crate::{generated::QuestName, specification_types::SINGLE_ONE_HP};
pub type StoryTransitionList = ConstList<StoryTransition>;

/// Event that can trigger a story transition
#[derive(Clone, Copy, Debug)]
pub struct StoryTransition {
    pub trigger: StoryTrigger,
    pub next_state: StoryState,
    pub actions: StoryActionList,
}

// TODO: Add trigger to watergate -> to duck slots (also make slote more visually appealing)
// TODO: manage mana
// TODO: Something is missing here. Something that introduces quests and something for letters. Maybe more.
impl StoryState {
    pub const fn transition(self, trigger: &StoryTrigger) -> Option<StoryTransition> {
        let transitions = self.guards();
        transitions.find(trigger)
    }
    /// List of legal transitions from this state
    pub const fn guards(self) -> StoryTransitionList {
        let mut out = StoryTransitionList::new();
        match self {
            Self::Initialized => {
                out = out.push(StoryTransition::on_dialogue(Self::ServantAccepted));
            }
            Self::ServantAccepted => {
                out = out.push(StoryTransition::on_building(
                    BuildingType::Temple,
                    Self::TempleBuilt,
                ));
            }
            Self::TempleBuilt => {
                out = out.push(StoryTransition::on_dialogue(Self::BuildingWatergate));
            }
            Self::BuildingWatergate => {
                out = out.push(StoryTransition::on_building(
                    BuildingType::Watergate,
                    Self::WatergateBuilt,
                ));
            }
            Self::WatergateBuilt => {
                out = out.push(
                    StoryTransition::on_dialogue(Self::VisitorArrived)
                        .with(StoryAction::SendHobo(SINGLE_ONE_HP)),
                );
            }
            Self::VisitorArrived => {
                out = out.push(
                    StoryTransition::on_dialogue(Self::VisitorArrived)
                        .with(StoryAction::StartQuest(QuestName::HelloWorld)),
                );
                out = out.push(
                    StoryTransition::after_quest(QuestName::HelloWorld, Self::FirstVisitorWelcomed)
                        .with(StoryAction::StartQuest(QuestName::CreateForest)),
                );
            }
            Self::FirstVisitorWelcomed => {
                out = out.push(
                    StoryTransition::after_quest(
                        QuestName::CreateForest,
                        Self::FirstVisitorWelcomed,
                    )
                    .with(StoryAction::StartQuest(QuestName::BuildBundligStation)),
                );
                out = out.push(
                    StoryTransition::after_quest(
                        QuestName::BuildBundligStation,
                        Self::FirstVisitorWelcomed,
                    )
                    .with(StoryAction::StartQuest(QuestName::UseBundligStation)),
                );
                out = out.push(StoryTransition::after_quest(
                    QuestName::UseBundligStation,
                    Self::GatheringSticks,
                ));
            }
            _ => {}
        }
        out
    }
}

impl StoryTransition {
    pub const fn new(trigger: StoryTrigger, next_state: StoryState) -> Self {
        Self {
            trigger,
            next_state,
            actions: StoryActionList::new(),
        }
    }
    pub const fn on_dialogue(next_state: StoryState) -> Self {
        Self {
            trigger: StoryTrigger::DialogueStoryTrigger,
            next_state,
            actions: StoryActionList::new(),
        }
    }
    pub const fn on_building(bt: BuildingType, next_state: StoryState) -> Self {
        Self {
            trigger: StoryTrigger::BuildingBuilt(bt),
            next_state,
            actions: StoryActionList::new(),
        }
    }
    pub const fn on_choice(choice: StoryChoice, next_state: StoryState) -> Self {
        Self {
            trigger: StoryTrigger::DialogueChoice(choice),
            next_state,
            actions: StoryActionList::new(),
        }
    }
    pub const fn after_quest(quest: QuestName, next_state: StoryState) -> Self {
        Self {
            trigger: StoryTrigger::FinishedQuest(quest),
            next_state,
            actions: StoryActionList::new(),
        }
    }
    pub const fn with(mut self, action: StoryAction) -> Self {
        self.actions = self.actions.push(action);
        self
    }
    pub const fn is_trigger(&self, trigger: &StoryTrigger) -> bool {
        // Yeah, I whish I knew a better way doing that... (Maybe PartialEq will eventually get a const version)
        match (&self.trigger, trigger) {
            (StoryTrigger::DialogueStoryTrigger, StoryTrigger::DialogueStoryTrigger) => true,
            (StoryTrigger::DialogueChoice(a), StoryTrigger::DialogueChoice(b)) => a.const_eq(*b),
            (StoryTrigger::FinishedQuest(a), StoryTrigger::FinishedQuest(b)) => a.const_eq(*b),
            (StoryTrigger::BuildingBuilt(a), StoryTrigger::BuildingBuilt(b)) => a.const_eq(*b),
            _ => false,
        }
    }
}

// Pseudo const-trait
impl BuildingType {
    pub const fn const_eq(self, other: Self) -> bool {
        self as usize == other as usize
    }
}
