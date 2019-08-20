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
    units,
    attacks,
    attacks_to_units,
    buildings,
    resources,
    tasks,
    streams
};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Unit_type")]
pub enum UnitType {
    Basic,
    Hero,
}
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "enum_utils", derive(EnumIter, Display))]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Unit_color")]
pub enum UnitColor {
    Yellow,
    White,
    Camo,
}


#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct Unit {
    pub id: i64,
    pub home: i64,
    pub x: i32,
    pub y: i32,
    pub unit_type: UnitType,
    pub color: Option<UnitColor>,
    pub hp: i64,
    pub speed: f32, // in unit lengths per second
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "units"]
pub struct NewUnit {
    pub home: i64,
    pub x: i32,
    pub y: i32,
    pub unit_type: UnitType,
    pub color: Option<UnitColor>,
    pub hp: i64,
    pub speed: f32,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable)]
pub struct Attack {
    pub id: i64,
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "attacks"]
pub struct NewAttack {
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable,Insertable)]
#[table_name = "attacks_to_units"]
pub struct AttackToUnit {
    pub attack_id: i64,
    pub unit_id: i64,
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
}

#[cfg(feature = "sql_db")]
#[derive(Queryable, Debug)]
pub struct Building {
    pub id: i64,
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>, 
    pub attack_power: Option<f32>, 
    pub attacks_per_cycle: Option<i32>,
    pub creation: NaiveDateTime,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable, Debug)]
#[table_name = "buildings"]
pub struct NewBuilding {
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>, 
    pub attack_power: Option<f32>, 
    pub attacks_per_cycle: Option<i32>,
    pub creation: NaiveDateTime,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[cfg(feature = "sql_db")]
pub struct Task {
    pub id: i64,
    pub unit_id: i64,
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
    pub start_time: NaiveDateTime,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub unit_id: i64,
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
    pub start_time: Option<NaiveDateTime>,
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