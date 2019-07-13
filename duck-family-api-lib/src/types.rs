//! Duplicated types
//! Ideally, these would all be read from the db_lib::models module directly.
//! Unfortunately, when I try loading that module the linker crashes and after spending an afternoon 
//! I decided to drop that dependency in the API lib and then add boilerplate-mappings in the db_lib.

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Display)]
pub enum ResourceType {
    Sticks,
    Logs,
    Feathers,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, EnumIter, Display)]
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