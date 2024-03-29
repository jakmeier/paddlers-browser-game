use serde::{Deserialize, Serialize};

#[cfg(feature = "sql_db")]
use ::diesel_derive_enum::DbEnum;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Story_state_type")]
pub enum StoryState {
    Initialized,
    ServantAccepted,
    TempleBuilt,
    BuildingWatergate,
    WatergateBuilt,
    VisitorQueued,
    VisitorArrived,
    WelcomeVisitorQuestStarted,
    FirstVisitorWelcomed,
    PickingPrimaryCivBonus,
    SolvingPrimaryCivQuestPartA,
    SolvingPrimaryCivQuestPartB,
    UnlockingInvitationPathA,
    UnlockingInvitationPathB,
    DialogueBalanceA,
    DialogueBalanceB,
    SolvingSecondaryQuestA,
    SolvingSecondaryQuestB,
    AllDone,
}
