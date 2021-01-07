use crate::game::components::NetObj;
use crate::prelude::*;
use chrono::NaiveDateTime;
use graphql_client::GraphQLQuery;
use paddlers_shared_lib::graphql_types;
use paddlers_shared_lib::models::*;
use specs::prelude::*;

pub use serde::Deserialize;
type GqlTimestamp = String;

pub fn parse_timestamp(s: &String) -> NaiveDateTime {
    timestamp(s).to_chrono()
}
fn timestamp(s: &String) -> graphql_types::GqlTimestamp {
    graphql_types::GqlTimestamp::from_string(s).unwrap()
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/attacks_query.graphql"
)]
pub struct AttacksQuery;
pub type AttacksResponse = attacks_query::ResponseData;
pub type HoboEffect = attacks_query::AttacksQueryVillageAttacksUnitsHoboEffects;
pub type HoboAttribute = attacks_query::HoboAttributeType;

impl attacks_query::AttacksQueryVillageAttacks {
    #[allow(dead_code)]
    pub fn departure(&self) -> chrono::NaiveDateTime {
        timestamp(&self.departure).to_chrono()
    }
    #[allow(dead_code)]
    pub fn arrival(&self) -> chrono::NaiveDateTime {
        timestamp(&self.arrival).to_chrono()
    }
}

impl Into<HoboAttributeType> for &HoboAttribute {
    fn into(self) -> HoboAttributeType {
        match self {
            HoboAttribute::HEALTH => HoboAttributeType::Health,
            HoboAttribute::SPEED => HoboAttributeType::Speed,
            HoboAttribute::Other(_) => panic!("Unexpected attribute"),
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/buildings_query.graphql"
)]
pub struct BuildingsQuery;
pub type BuildingsResponse = buildings_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/volatile_village_info_query.graphql"
)]
pub struct VolatileVillageInfoQuery;
pub type VolatileVillageInfoResponse = volatile_village_info_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/village_units_query.graphql"
)]
pub struct VillageUnitsQuery;
pub type VillageUnitsResponse = village_units_query::ResponseData;
pub type WorkerResponse = Vec<village_units_query::VillageUnitsQueryVillageWorkers>;
#[allow(dead_code)]
pub type VillageUnitsTask = village_units_query::VillageUnitsQueryVillageWorkersTasks;
pub type VillageUnitsTaskType = village_units_query::TaskType;
pub type VillageUnitsAbilityType = village_units_query::AbilityType;

use crate::game::units::workers::WorkerTask;
impl VillageUnitsTask {
    pub fn create(
        &self,
        net_ids: &ReadStorage<NetObj>,
        entities: &Entities,
    ) -> PadlResult<WorkerTask> {
        let target = if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        } else {
            None
        };
        Ok(WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).to_chrono(),
            target,
        })
    }
}
impl Into<AbilityType> for &VillageUnitsAbilityType {
    fn into(self) -> AbilityType {
        match self {
            VillageUnitsAbilityType::WORK => AbilityType::Work,
            VillageUnitsAbilityType::WELCOME => AbilityType::Welcome,
            VillageUnitsAbilityType::Other(_) => panic!("Unexpected ability"),
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/worker_tasks_query.graphql"
)]
pub struct WorkerTasksQuery;
pub type WorkerTasksRawResponse = worker_tasks_query::ResponseData;
pub type WorkerTasksResponse = worker_tasks_query::WorkerTasksQueryWorker;
#[allow(dead_code)]
pub type WorkerTaskEx = worker_tasks_query::WorkerTasksQueryWorkerTasks;
pub type WorkerTaskType = worker_tasks_query::TaskType;

impl WorkerTaskEx {
    pub fn create(
        &self,
        net_ids: &ReadStorage<NetObj>,
        entities: &Entities,
    ) -> PadlResult<WorkerTask> {
        let target = if let Some(id) = self.hobo_target {
            Some(NetObj::lookup_hobo(id, net_ids, entities)?)
        } else {
            None
        };
        Ok(WorkerTask {
            task_type: (&self.task_type).into(),
            position: (self.x as usize, self.y as usize),
            start_time: timestamp(&self.start_time).to_chrono(),
            target,
        })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/hobos_query.graphql"
)]
pub struct HobosQuery;
pub type HobosQueryRawResponse = hobos_query::ResponseData;
pub type HobosQueryResponseVillage = hobos_query::HobosQueryVillage;
pub type HobosQueryResponse = Vec<hobos_query::HobosQueryVillageHobos>;
pub type HobosQueryUnitColor = hobos_query::UnitColor;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_query.graphql"
)]
pub struct PlayerQuery;
pub type PlayerQueryRawResponse = player_query::ResponseData;
pub type PlayerQueryResponse = player_query::PlayerQueryPlayer;
pub type PlayerStoryState = player_query::StoryState;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/map_query.graphql"
)]
pub struct MapQuery;
pub type MapResponse = map_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_villages_query.graphql"
)]
pub struct PlayerVillagesQuery;
pub type PlayerVillagesRawResponse = player_villages_query::ResponseData;
pub type PlayerVillagesResponse = player_villages_query::PlayerVillagesQueryPlayer;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/player_quests_query.graphql"
)]
pub struct PlayerQuestsQuery;
pub type QuestsRawResponse = player_quests_query::ResponseData;
pub type QuestsResponse = Vec<player_quests_query::PlayerQuestsQueryPlayerQuests>;
pub type PlayerQuest = player_quests_query::PlayerQuestsQueryPlayerQuests;

use paddlers_shared_lib::models::BuildingType;
impl Into<BuildingType> for &player_quests_query::BuildingType {
    fn into(self) -> BuildingType {
        match self {
            player_quests_query::BuildingType::RED_FLOWERS => BuildingType::RedFlowers,
            player_quests_query::BuildingType::BLUE_FLOWERS => BuildingType::BlueFlowers,
            player_quests_query::BuildingType::TREE => BuildingType::Tree,
            player_quests_query::BuildingType::BUNDLING_STATION => BuildingType::BundlingStation,
            player_quests_query::BuildingType::SAW_MILL => BuildingType::SawMill,
            player_quests_query::BuildingType::PRESENT_A => BuildingType::PresentA,
            player_quests_query::BuildingType::PRESENT_B => BuildingType::PresentB,
            player_quests_query::BuildingType::TEMPLE => BuildingType::Temple,
            player_quests_query::BuildingType::SINGLE_NEST => BuildingType::SingleNest,
            player_quests_query::BuildingType::TRIPLE_NEST => BuildingType::TripleNest,
            player_quests_query::BuildingType::Other(_) => panic!("Unexpected BuildingType"),
        }
    }
}
impl Into<TaskType> for &player_quests_query::TaskType {
    fn into(self) -> TaskType {
        match self {
            player_quests_query::TaskType::IDLE => TaskType::Idle,
            player_quests_query::TaskType::WALK => TaskType::Walk,
            player_quests_query::TaskType::GATHER_STICKS => TaskType::GatherSticks,
            player_quests_query::TaskType::CHOP_TREE => TaskType::ChopTree,
            player_quests_query::TaskType::DEFEND => TaskType::Defend,
            player_quests_query::TaskType::WELCOME_ABILITY => TaskType::WelcomeAbility,
            player_quests_query::TaskType::COLLECT_REWARD => TaskType::CollectReward,
            player_quests_query::TaskType::Other(_) => panic!("Unexpected task type"),
        }
    }
}

use paddlers_shared_lib::models::TaskType;
impl Into<TaskType> for &WorkerTaskType {
    fn into(self) -> TaskType {
        match self {
            WorkerTaskType::IDLE => TaskType::Idle,
            WorkerTaskType::WALK => TaskType::Walk,
            WorkerTaskType::GATHER_STICKS => TaskType::GatherSticks,
            WorkerTaskType::CHOP_TREE => TaskType::ChopTree,
            WorkerTaskType::DEFEND => TaskType::Defend,
            WorkerTaskType::WELCOME_ABILITY => TaskType::WelcomeAbility,
            WorkerTaskType::COLLECT_REWARD => TaskType::CollectReward,
            WorkerTaskType::Other(_) => panic!("Unexpected task type"),
        }
    }
}
impl Into<TaskType> for WorkerTaskType {
    fn into(self) -> TaskType {
        (&self).into()
    }
}
impl Into<TaskType> for &VillageUnitsTaskType {
    fn into(self) -> TaskType {
        match self {
            VillageUnitsTaskType::IDLE => TaskType::Idle,
            VillageUnitsTaskType::WALK => TaskType::Walk,
            VillageUnitsTaskType::GATHER_STICKS => TaskType::GatherSticks,
            VillageUnitsTaskType::CHOP_TREE => TaskType::ChopTree,
            VillageUnitsTaskType::DEFEND => TaskType::Defend,
            VillageUnitsTaskType::WELCOME_ABILITY => TaskType::WelcomeAbility,
            VillageUnitsTaskType::COLLECT_REWARD => TaskType::CollectReward,
            VillageUnitsTaskType::Other(_) => panic!("Unexpected task type"),
        }
    }
}
impl Into<TaskType> for VillageUnitsTaskType {
    fn into(self) -> TaskType {
        (&self).into()
    }
}

impl Into<UnitColor> for &attacks_query::UnitColor {
    fn into(self) -> UnitColor {
        match self {
            attacks_query::UnitColor::YELLOW => UnitColor::Yellow,
            attacks_query::UnitColor::WHITE => UnitColor::White,
            attacks_query::UnitColor::CAMO => UnitColor::Camo,
            attacks_query::UnitColor::PROPHET => UnitColor::Prophet,
            attacks_query::UnitColor::Other(_) => panic!("Unexpected unit color"),
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/leaderboard_query.graphql"
)]
pub struct LeaderboardQuery;
pub type LeaderboardRawResponse = leaderboard_query::ResponseData;
pub type LeaderboardResponse = Vec<leaderboard_query::LeaderboardQueryScoreboard>;

use paddlers_shared_lib::story::story_state::StoryState;
impl Into<StoryState> for &PlayerStoryState {
    fn into(self) -> StoryState {
        match self {
            PlayerStoryState::INITIALIZED => StoryState::Initialized,
            PlayerStoryState::SERVANT_ACCEPTED => StoryState::ServantAccepted,
            PlayerStoryState::TEMPLE_BUILT => StoryState::TempleBuilt,
            PlayerStoryState::VISITOR_ARRIVED => StoryState::VisitorArrived,
            PlayerStoryState::FIRST_VISITOR_WELCOMED => StoryState::FirstVisitorWelcomed,
            PlayerStoryState::FLOWER_PLANTED => StoryState::FlowerPlanted,
            PlayerStoryState::MORE_HAPPY_VISITORS => StoryState::MoreHappyVisitors,
            PlayerStoryState::TREE_PLANTED => StoryState::TreePlanted,
            PlayerStoryState::STICK_GATHERING_STATION_BUILD => {
                StoryState::StickGatheringStationBuild
            }
            PlayerStoryState::GATHERING_STICKS => StoryState::GatheringSticks,
            PlayerStoryState::Other(_) => panic!("Unexpected story state"),
        }
    }
}
impl Into<StoryState> for PlayerStoryState {
    fn into(self) -> StoryState {
        (&self).into()
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "api/schema.json",
    query_path = "api/queries/reports_query.graphql"
)]
pub struct ReportsQuery;
pub type ReportsResponse = reports_query::ResponseData;
