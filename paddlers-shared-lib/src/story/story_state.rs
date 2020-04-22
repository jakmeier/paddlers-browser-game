//! Each player is in one StoryState, depending on which story texts he clicked through already.
//! It should be a (mostly) linear progression through these states.
//! These are stored in the database and provided as PlayerInfo to the frontend.
//!
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
