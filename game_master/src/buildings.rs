use db_lib::models::*;

pub struct BuildingFactory;

impl BuildingFactory {
    pub fn new(typ: BuildingType, pos: (usize, usize)) -> NewBuilding {
        let mut new_building = NewBuilding {
            x: pos.0 as i32,
            y: pos.1 as i32,
            building_type: typ,
            building_range: None, 
            attack_power: None, 
            attacks_per_cycle: None,
        };
        match typ {
            BuildingType::BlueFlowers => {
                new_building.building_range = Some(5.0);
                new_building.attack_power = Some(1.0);
            },
            BuildingType::RedFlowers => {
                new_building.building_range = Some(1.0);
                new_building.attack_power = Some(3.0);
            },
        }
        new_building
    }
}