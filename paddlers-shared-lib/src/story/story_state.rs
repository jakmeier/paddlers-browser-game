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
    VisitorQueued,
    VisitorArrived,
    WelcomeVisitorQuestStarted,
    FirstVisitorWelcomed,
    PickingPrimaryCivBonus,
    SolvingPrimaryCivQuestPartA,
    SolvingPrimaryCivQuestPartB,
    DialogueBalanceA,
    DialogueBalanceB,
    SolvingSecondaryQuestA,
    SolvingSecondaryQuestB,
    AllDone,
}
