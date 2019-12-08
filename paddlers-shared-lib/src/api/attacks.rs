use serde::*;
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttackDescriptor {
    pub from: VillageKey,
    pub to: (i32,i32),
    pub units: Vec<Troop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Troop {
    pub color: UnitColor, 
    pub typ: UnitType, 
    pub count: u32,
}