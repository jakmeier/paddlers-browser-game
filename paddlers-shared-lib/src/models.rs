use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[cfg(feature = "sql_db")]
use ::diesel_derive_enum;
#[cfg(feature = "sql_db")]
use diesel_derive_enum::DbEnum;
#[cfg(feature = "sql_db")]
pub use resources::dsl;

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable, Identifiable)]
pub struct Unit {
    pub id: i64,
    pub sprite: String,
    pub hp: i64,
    pub speed: f32,
}

#[cfg(feature = "sql_db")]
#[derive(Insertable)]
#[table_name = "units"]
pub struct NewUnit<'a> {
    pub sprite: &'a str,
    pub hp: i64,
    pub speed: f32,
}

#[cfg(feature = "sql_db")]
use super::schema::attacks;

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
use super::schema::attacks_to_units;

#[cfg(feature = "sql_db")]
#[derive(Debug, Queryable,Insertable)]
#[table_name = "attacks_to_units"]
pub struct AttackToUnit {
    pub attack_id: i64,
    pub unit_id: i64,
}

#[cfg(feature = "sql_db")]
use super::schema::units;
#[cfg(feature = "sql_db")]
#[allow(non_camel_case_types)]
pub type UNIT_ALL_COLUMNS_T =  (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);
#[cfg(feature = "sql_db")]
pub const UNIT_ALL_COLUMNS: UNIT_ALL_COLUMNS_T = (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, EnumIter, Display)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", derive(DbEnum), DieselType = "Building_type")]
pub enum BuildingType {
    BlueFlowers,
    RedFlowers,
    Tree,
}
impl BuildingType {
    pub fn all() -> BuildingTypeIter {
        use strum::IntoEnumIterator;
        BuildingType::iter()
    }
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
use super::schema::buildings;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumIter, Display)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Resource_type", derive(DbEnum))]
pub enum ResourceType {
    Sticks,
    Logs,
    Feathers,
}

#[cfg(feature = "sql_db")]
use super::schema::resources;

#[cfg(feature = "sql_db")]
#[derive(Identifiable, Insertable, Queryable, Debug)]
#[table_name = "resources"]
#[primary_key(resource_type)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: i64,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumIter, Display)]
#[cfg_attr(feature = "graphql", derive(juniper::GraphQLEnum))]
#[cfg_attr(feature = "sql_db", DieselType = "Task_type", derive(DbEnum))]
pub enum TaskType {
    Idle,
    Walk,
    Defend,
    GatherSticks,
    ChopTree, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sql_db", derive(Queryable))]
pub struct Task {
    pub id: i64,
    pub unit_id: i64,
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
    pub start_time: NaiveDateTime,
}

pub struct NewTask {
    pub task_type: TaskType,
    pub x: i32,
    pub y: i32,
}