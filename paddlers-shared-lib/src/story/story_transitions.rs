//! Each player is in one StoryState, depending on the tutorial/story progression.
//!
//! The StoryState values are stored in the database per player and provided as PlayerInfo to the frontend.
//! Transitions are performed in the game-master when a StoryTrigger happens, following the FSM definied in `fn transition`.
//! In each transition, a set of StoryActions is also performed in the game-master and/or frontend.

use super::{story_action::StoryActionList, story_state::StoryState, story_trigger::StoryChoice};
use crate::story::story_trigger::StoryTrigger;
use crate::{civilization::CivilizationPerk, story::story_action::StoryAction};
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
                    StoryTransition::on_dialogue(Self::VisitorQueued)
                        .with(StoryAction::SendHobo(SINGLE_ONE_HP)),
                );
            }
            Self::VisitorQueued => {
                out = out.push(StoryTransition {
                    trigger: StoryTrigger::LetVisitorIn,
                    next_state: Self::VisitorArrived,
                    actions: StoryActionList::new(),
                });
            }
            Self::VisitorArrived => {
                out = out.push(
                    StoryTransition::on_dialogue(Self::WelcomeVisitorQuestStarted)
                        .with(StoryAction::StartQuest(QuestName::HelloWorld))
                        .with(StoryAction::AddMana(10)),
                );
            }
            Self::WelcomeVisitorQuestStarted => {
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
                    .with(StoryAction::StartQuest(QuestName::BuildBundlingStation)),
                );
                out = out.push(
                    StoryTransition::after_quest(
                        QuestName::BuildBundlingStation,
                        Self::FirstVisitorWelcomed,
                    )
                    .with(StoryAction::StartQuest(QuestName::UseBundlingStation)),
                );
                out = out.push(StoryTransition::after_quest(
                    QuestName::UseBundlingStation,
                    Self::PickingPrimaryCivBonus,
                ));
            }
            Self::PickingPrimaryCivBonus => {
                out = out.push(
                    StoryTransition::on_choice(
                        StoryChoice::new(0),
                        Self::SolvingPrimaryCivQuestPartA,
                    )
                    .with(StoryAction::StartQuest(QuestName::Socialize)),
                );
                out = out.push(
                    StoryTransition::on_choice(
                        StoryChoice::new(1),
                        Self::SolvingPrimaryCivQuestPartA,
                    )
                    .with(StoryAction::StartQuest(QuestName::BuildNest))
                    .with(StoryAction::UnlockPerk(CivilizationPerk::NestBuilding)),
                );
            }
            Self::SolvingPrimaryCivQuestPartA => {
                out = out.push(
                    StoryTransition::after_quest(
                        QuestName::Socialize,
                        Self::SolvingPrimaryCivQuestPartB,
                    )
                    .with(StoryAction::StartQuest(QuestName::SocializeMore))
                    .with(StoryAction::UnlockPerk(CivilizationPerk::Invitation)),
                );
                out = out.push(
                    StoryTransition::after_quest(
                        QuestName::BuildNest,
                        Self::SolvingPrimaryCivQuestPartB,
                    )
                    .with(StoryAction::StartQuest(QuestName::GrowPopulation)),
                );
            }
            Self::SolvingPrimaryCivQuestPartB => {
                out = out.push(StoryTransition::after_quest(
                    QuestName::SocializeMore,
                    Self::DialogueBalanceA,
                ));
                out = out.push(StoryTransition::after_quest(
                    QuestName::GrowPopulation,
                    Self::DialogueBalanceB,
                ));
            }
            Self::DialogueBalanceA => {
                out = out.push(
                    StoryTransition::on_dialogue(Self::SolvingSecondaryQuestA)
                        .with(StoryAction::StartQuest(QuestName::GrowPopulation)),
                );
            }
            Self::DialogueBalanceB => {
                out = out.push(
                    StoryTransition::on_dialogue(Self::SolvingSecondaryQuestB)
                        .with(StoryAction::UnlockPerk(CivilizationPerk::Invitation))
                        .with(StoryAction::StartQuest(QuestName::SocializeMore)),
                );
            }
            Self::SolvingSecondaryQuestA => {
                out = out.push(StoryTransition::after_quest(
                    QuestName::GrowPopulation,
                    Self::AllDone,
                ));
            }
            Self::SolvingSecondaryQuestB => {
                out = out.push(StoryTransition::after_quest(
                    QuestName::SocializeMore,
                    Self::AllDone,
                ));
            }
            Self::AllDone => {}
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
            (StoryTrigger::LetVisitorIn, StoryTrigger::LetVisitorIn) => true,
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn let_visitor_in() {
        assert!(StoryState::VisitorQueued.guards().len() == 1);
        println!("Guards: {:?}", StoryState::VisitorQueued.guards());
        assert!(StoryState::VisitorQueued
            .transition(&StoryTrigger::LetVisitorIn)
            .is_some());
    }
}
