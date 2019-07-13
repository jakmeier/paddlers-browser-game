use duck_family_db_lib::models::*;
use duck_family_api_lib::types;
use duck_family_api_lib::attributes::*;

pub struct BuildingFactory;

impl BuildingFactory {
    pub fn new(typ: types::BuildingType, pos: (usize, usize)) -> NewBuilding {
        NewBuilding {
            x: pos.0 as i32,
            y: pos.1 as i32,
            building_type: typ.into(),
            building_range: typ.range(), 
            attack_power: typ.attack_power(), 
            attacks_per_cycle: typ.attacks_per_cycle(),
        }
    }
}