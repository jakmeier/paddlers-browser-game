use serde::{Serialize, Deserialize};

#[cfg(feature = "sql_db")]
use chrono::NaiveDateTime;

// Reexport
#[cfg(feature = "sql_db")]
pub use resources::dsl;

#[cfg(feature = "sql_db")]
use ::diesel_derive_enum::DbEnum;

#[cfg(feature = "sql_db")]
use super::schema::{
    hobos,
    workers,
    worker_flags,
    attacks,
    attacks_to_hobos,
    buildings,
    resources,
    tasks,
    streams,
    players,
    villages,
    abilities,
    effects,
};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Unit_type")]
pub enum UnitType {
    Basic,
    Hero,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Unit_color")]
pub enum UnitColor {
    Yellow,
    White,
    Camo,
    Prophet,
}


#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable, AsChangeset, Clone)]
#[table_name = "players"]
pub struct Player {
    pub id: i64,
    pub uuid: uuid::Uuid,
    pub karma: i64,
    pub display_name: String,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub uuid: uuid::Uuid,
    pub karma: i64,
    pub display_name: String,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Worker {
    pub id: i64,
    pub home: i64,
    pub x: i32,
    pub y: i32,
    pub unit_type: UnitType,
    pub color: Option<UnitColor>,
    pub speed: f32, // in unit lengths per second
    pub mana: Option<i32>,
    pub level: i32,
    pub exp: i32,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "workers"]
pub struct NewWorker {
    pub home: i64,
    pub x: i32,
    pub y: i32,
    pub unit_type: UnitType,
    pub color: Option<UnitColor>,
    pub speed: f32,
    pub mana: Option<i32>,
    pub level: i32,
    pub exp: i32,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Hobo {
    pub id: i64,
    pub home: i64,
    pub color: Option<UnitColor>,
    pub speed: f32,
    pub hp: i64,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "hobos"]
pub struct NewHobo {
    pub hp: i64,
    pub home: i64,
    pub color: Option<UnitColor>,
    pub speed: f32,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable)]
pub struct Attack {
    pub id: i64,
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
    pub origin_village_id: i64,
    pub destination_village_id: i64,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "attacks"]
pub struct NewAttack {
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
    pub origin_village_id: i64,
    pub destination_village_id: i64,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable,Insertable)]
#[table_name = "attacks_to_hobos"]
pub struct AttackToHobo {
    pub attack_id: i64,
    pub hobo_id: i64,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Building_type")]
pub enum BuildingType {
    BlueFlowers,
    RedFlowers,
    Tree,
    BundlingStation,
    SawMill,
    PresentA,
    PresentB,
    Temple,
}

#[cfg(feature = "sql_db")]
#[derive(Queryable, Debug)]
pub struct Building {
    pub id: i64,
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>,
    pub attack_power: Option<i32>,
    pub attacks_per_cycle: Option<i32>,
    pub creation: NaiveDateTime,
    pub village_id: i64,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable, Debug)]
#[table_name = "buildings"]
pub struct NewBuilding {
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>, 
    pub attack_power: Option<i32>, 
    pub attacks_per_cycle: Option<i32>,
    pub creation: NaiveDateTime,
    pub village_id: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Resource_type", derive(DbEnum))]
pub enum ResourceType {
    Sticks,
    Logs,
    Feathers,
}

#[cfg(feature = "sql_db")]
#[derive(Identifiable, Insertable, Queryable, Debug)]
#[table_name = "resources"]
#[primary_key(resource_type)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: i64,
    pub village_id: i64,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Task_type", derive(DbEnum))]
pub enum TaskType {
    Idle,
    Walk,
    Defend,
    GatherSticks,
    ChopTree,
    WelcomeAbility,
    CollectReward,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[cfg(feature = "sql_db")]
pub struct Task {
    pub id: i64,
    pub worker_id: i64,
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
    pub start_time: NaiveDateTime,
    pub target_hobo_id: Option<i64>,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub worker_id: i64,
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
    pub start_time: Option<NaiveDateTime>,
    pub target_hobo_id: Option<i64>,
}

#[derive(Debug, Clone, Queryable, Identifiable)]
#[cfg(feature = "sql_db")]
pub struct Stream {
    pub id: i64,
    pub start_x: f32,
    pub control_points: Vec<f32>,
}

#[derive(Insertable, Debug)]
#[cfg(feature = "sql_db")]
#[table_name = "streams"]
pub struct NewStream {
    pub start_x: f32,
    pub control_points: Vec<f32>,
}

#[derive(Debug, Clone, Copy, Queryable, Identifiable)]
#[cfg(feature = "sql_db")]
pub struct Village {
    pub id: i64,
    pub x: f32,
    pub y: f32,
    pub stream_id: i64,
    pub player_id: Option<i64>,
    pub faith: i16,
}

#[derive(Insertable, Debug)]
#[cfg(feature = "sql_db")]
#[table_name = "villages"]
pub struct NewVillage {
    pub x: f32,
    pub y: f32,
    pub stream_id: i64,
    pub player_id: Option<i64>,
    pub faith: Option<i16>,
}

#[derive(Debug, Clone, Copy, Queryable, AsChangeset)]
#[cfg(feature = "sql_db")]
#[table_name = "abilities"]
pub struct Ability {
    pub ability_type: AbilityType,
    pub worker_id: i64,
    pub last_used: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[cfg(feature = "sql_db")]
#[table_name = "abilities"]
pub struct NewAbility {
    pub ability_type: AbilityType,
    pub worker_id: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Ability_type", derive(DbEnum))]
/// Abilities are attributes of worker and hero units.
/// They are closely related to Tasks but there is no one-to-one correspondence.
pub enum AbilityType {
    Work,
    Welcome,
}

#[derive(Debug, Clone, Copy, Queryable)]
#[cfg(feature = "sql_db")]
pub struct Effect {
    pub id: i64,
    pub hobo_id: i64,
    pub attribute: HoboAttributeType,
    pub strength: Option<i32>,
    pub start_time: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[cfg(feature = "sql_db")]
#[table_name = "effects"]
pub struct NewEffect {
    pub hobo_id: i64,
    pub attribute: HoboAttributeType,
    pub strength: Option<i32>,
    pub start_time: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Hobo_attribute_type", derive(DbEnum))]
/// Describes an attribute of a hobo
pub enum HoboAttributeType {
    Health,
    Speed,
}

#[derive(Debug, Clone, Copy, Queryable, Insertable)]
#[cfg(feature = "sql_db")]
pub struct WorkerFlag {
    pub worker_id: i64,
    pub flag_type: WorkerFlagType,
    pub last_update: NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Worker_flag_type", derive(DbEnum))]
pub enum WorkerFlagType {
    ManaRegeneration,
    Work,
}