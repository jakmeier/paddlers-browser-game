use crate::db::DB;
use duck_family_db_lib::models::*;
use crate::buildings::BuildingFactory;
use crate::StringErr;

pub struct Price(pub Vec<(ResourceType, i64)>);
trait Cost {
    fn cost(&self) -> Vec<(ResourceType, i64)>;
    fn price(&self) -> Price { Price(self.cost()) }
}

impl DB {
    // TODO: This should be with village/user parameters
    pub fn try_buy_building(&self, typ: BuildingType, pos: (usize ,usize)) -> StringErr {
        println!("Buying building");
        
        self.building_has_space(typ, pos)
            .map(
                |_| self.try_spend(&typ.price())
            ).map(
                |_| {
                    self.insert_building(&BuildingFactory::new(typ, pos));
                }
            )
    }

    fn building_has_space(&self,  _typ: BuildingType, _pos: (usize ,usize)) -> StringErr {
        // TODO Check if bulding slot is empty
        Ok(())
    }
}

impl Cost for BuildingType {
    fn cost(&self) -> Vec<(ResourceType, i64)> {
        match self {
            BuildingType::BlueFlowers 
                => vec![(ResourceType::Feathers, 20)],
            BuildingType::RedFlowers
                => vec![
                    (ResourceType::Feathers, 50),
                    (ResourceType::Sticks, 5),
                ],
        }
    }
}
