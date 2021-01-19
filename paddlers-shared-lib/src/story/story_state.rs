//! Each player is in one StoryState, depending on the tutorial/story progression.
//!
//! The StoryState values are stored in the database per player and provided as PlayerInfo to the frontend.
//! Transitions are performed in the game-master when a StoryTrigger happens, following the FSM definied in `fn transition`.
//! In each transition, a set of StoryActions is also performed in the game-master and/or frontend.
use crate::generated::QuestName;
use crate::prelude::BuildingType;
use crate::story::story_action::StoryAction;
use crate::story::story_trigger::StoryTrigger;
use serde::{Deserialize, Serialize};

#[cfg(feature = "sql_db")]
use ::diesel_derive_enum::DbEnum;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Story_state_type", derive(DbEnum))]
pub enum StoryState {
    Initialized,
    ServantAccepted,
    TempleBuilt,
    BuildingWatergate,
    WatergateBuilt,
    VisitorArrived,
    FirstVisitorWelcomed,
    FlowerPlanted,
    MoreHappyVisitors,
    TreePlanted,
    StickGatheringStationBuild,
    GatheringSticks,
    PickingPrimaryCivBonus,
    SolvingPrimaryCivQuestPartA,
    SolvingPrimaryCivQuestPartB,
    PickingSecondaryCivBonus,
    SolvingSecondaryQuestPartA,
    SolvingSecondaryQuestPartB,
    AllDone,
}

use StoryTrigger::*;
impl StoryState {
    pub const fn transition(self, trigger: &StoryTrigger) -> (StoryState, Option<StoryAction>) {
        let mut next_state = self;
        let mut action = None;
        match (self, trigger) {
            (Self::Initialized, DialogueStoryTrigger) => {
                next_state = Self::ServantAccepted;
            }
            (Self::ServantAccepted, BuildingBuilt(BuildingType::Temple)) => {
                next_state = Self::TempleBuilt;
            }
            (Self::TempleBuilt, DialogueStoryTrigger) => {
                next_state = Self::BuildingWatergate;
            }
            (Self::BuildingWatergate, BuildingBuilt(BuildingType::Watergate)) => {
                next_state = Self::WatergateBuilt;
            }
            (Self::WatergateBuilt, DialogueStoryTrigger) => {
                next_state = Self::VisitorArrived;
                action = Some(StoryAction::SendHobo);
            }
            // TODO: Something is missing here. Something that introduces quests and something for letters. Maybe more.
            (Self::VisitorArrived, DialogueStoryTrigger) => {
                action = Some(StoryAction::StartQuest(QuestName::HelloWorld));
            }
            (Self::VisitorArrived, StoryTrigger::FinishedQuest(QuestName::HelloWorld)) => {
                action = Some(StoryAction::StartQuest(QuestName::CreateForest));
                next_state = Self::FirstVisitorWelcomed;
            }
            (Self::FirstVisitorWelcomed, StoryTrigger::FinishedQuest(QuestName::CreateForest)) => {
                action = Some(StoryAction::StartQuest(QuestName::CreateForest));
                next_state = Self::TreePlanted;
            }
            (Self::TreePlanted, StoryTrigger::FinishedQuest(QuestName::BuildBundligStation)) => {
                action = Some(StoryAction::StartQuest(QuestName::UseBundligStation));
                next_state = Self::StickGatheringStationBuild;
            }
            (
                Self::StickGatheringStationBuild,
                StoryTrigger::FinishedQuest(QuestName::UseBundligStation),
            ) => {
                next_state = Self::GatheringSticks;
            }
            (Self::GatheringSticks, _) => {}
            (Self::FlowerPlanted, _) => {}
            (Self::MoreHappyVisitors, _) => {}
            (_, _) => { /* NOP */ }
        }
        (next_state, action)
    }
}
