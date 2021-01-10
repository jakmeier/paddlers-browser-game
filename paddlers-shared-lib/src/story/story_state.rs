//! Each player is in one StoryState, depending on the tutorial/story progression.
//!
//! The StoryState values are stored in the database per player and provided as PlayerInfo to the frontend.
//! Transitions are performed in the game-master when a StoryTrigger happens, following the FSM definied in `fn transition`.
//! In each transition, a set of StoryActions is also performed in the game-master and/or frontend.
use crate::generated::Quest;
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
    VisitorArrived,
    FirstVisitorWelcomed,
    FlowerPlanted,
    MoreHappyVisitors,
    TreePlanted,
    StickGatheringStationBuild,
    GatheringSticks,
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
                action = Some(StoryAction::SendHobo);
            }
            (Self::TempleBuilt, DialogueStoryTrigger) => {
                next_state = Self::VisitorArrived;
            }
            (Self::VisitorArrived, DialogueStoryTrigger) => {
                action = Some(StoryAction::StartQuest(Quest::HelloWorld));
            }
            (Self::VisitorArrived, StoryTrigger::FinishedQuest(Quest::HelloWorld)) => {
                action = Some(StoryAction::StartQuest(Quest::CreateForest));
                next_state = Self::FirstVisitorWelcomed;
            }
            (Self::FirstVisitorWelcomed, StoryTrigger::FinishedQuest(Quest::CreateForest)) => {
                action = Some(StoryAction::StartQuest(Quest::BuildBundligStation));
                next_state = Self::TreePlanted;
            }
            (Self::TreePlanted, StoryTrigger::FinishedQuest(Quest::BuildBundligStation)) => {
                action = Some(StoryAction::StartQuest(Quest::UseBundligStation));
                next_state = Self::StickGatheringStationBuild;
            }
            (Self::StickGatheringStationBuild, StoryTrigger::FinishedQuest(Quest::UseBundligStation)) => {
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
