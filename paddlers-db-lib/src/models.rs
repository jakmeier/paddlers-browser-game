use chrono::NaiveDateTime;
use ::diesel_derive_enum;
use diesel_derive_enum::DbEnum;
pub use resources::dsl;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Queryable, Identifiable)]
pub struct Unit {
    pub id: i64,
    pub sprite: String,
    pub hp: i64,
    pub speed: f32,
}
#[derive(Insertable)]
#[table_name = "units"]
pub struct NewUnit<'a> {
    pub sprite: &'a str,
    pub hp: i64,
    pub speed: f32,
}

use super::schema::attacks;
#[derive(Debug, Queryable, Identifiable)]
pub struct Attack {
    pub id: i64,
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}
#[derive(Insertable)]
#[table_name = "attacks"]
pub struct NewAttack {
    pub departure: NaiveDateTime,
    pub arrival: NaiveDateTime,
}

use super::schema::attacks_to_units;
#[derive(Debug, Queryable,Insertable)]
#[table_name = "attacks_to_units"]
pub struct AttackToUnit {
    pub attack_id: i64,
    pub unit_id: i64,
}

use super::schema::units;
#[allow(non_camel_case_types)]
pub type UNIT_ALL_COLUMNS_T =  (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);
pub const UNIT_ALL_COLUMNS: UNIT_ALL_COLUMNS_T = (
    units::id,
    units::sprite,
    units::hp,
    units::speed,
);

#[derive(Debug, PartialEq, DbEnum, Clone, Copy, Serialize, Deserialize, juniper::GraphQLEnum, EnumIter, Display)]
#[DieselType = "Building_type"]
pub enum BuildingType {
    BlueFlowers,
    RedFlowers,
}
impl BuildingType {
    pub fn all() -> BuildingTypeIter {
        use strum::IntoEnumIterator;
        BuildingType::iter()
    }
}
impl From<paddlers_api_lib::types::BuildingType> for BuildingType {
    fn from(input: paddlers_api_lib::types::BuildingType) -> Self {
        match input {
            paddlers_api_lib::types::BuildingType::BlueFlowers 
                => BuildingType::BlueFlowers,
            paddlers_api_lib::types::BuildingType::RedFlowers 
                => BuildingType::RedFlowers,
        }
    }
}
impl Into<paddlers_api_lib::types::BuildingType> for BuildingType {
    fn into(self) -> paddlers_api_lib::types::BuildingType {
        match self {
            BuildingType::BlueFlowers 
                => paddlers_api_lib::types::BuildingType::BlueFlowers,
            BuildingType::RedFlowers 
                => paddlers_api_lib::types::BuildingType::RedFlowers,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct Building {
    pub id: i64,
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>, 
    pub attack_power: Option<f32>, 
    pub attacks_per_cycle: Option<i32>,
}
use super::schema::buildings;
#[derive(Insertable, Debug)]
#[table_name = "buildings"]
pub struct NewBuilding {
    pub x: i32,
    pub y: i32,
    pub building_type: BuildingType,
    pub building_range: Option<f32>, 
    pub attack_power: Option<f32>, 
    pub attacks_per_cycle: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Hash, DbEnum, Clone, Copy, Serialize, Deserialize, juniper::GraphQLEnum, EnumIter, Display)]
#[allow(non_camel_case_types)]
#[DieselType = "Resource_type"]
pub enum ResourceType {
    Sticks,
    Logs,
    Feathers,
}
impl From<paddlers_api_lib::types::ResourceType> for ResourceType {
    fn from(input: paddlers_api_lib::types::ResourceType) -> Self {
        match input {
            paddlers_api_lib::types::ResourceType::Sticks 
                => ResourceType::Sticks,
            paddlers_api_lib::types::ResourceType::Logs 
                => ResourceType::Logs,
            paddlers_api_lib::types::ResourceType::Feathers 
                => ResourceType::Feathers,
        }
    }
}
impl Into<paddlers_api_lib::types::ResourceType> for ResourceType {
    fn into(self) -> paddlers_api_lib::types::ResourceType {
        match self {
            ResourceType::Sticks 
                => paddlers_api_lib::types::ResourceType::Sticks,
            ResourceType::Logs 
                => paddlers_api_lib::types::ResourceType::Logs,
            ResourceType::Feathers 
                => paddlers_api_lib::types::ResourceType::Feathers,
        }
    }
}

use super::schema::resources;
#[derive(Identifiable, Insertable, Queryable, Debug)]
#[table_name = "resources"]
#[primary_key(resource_type)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: i64,
}