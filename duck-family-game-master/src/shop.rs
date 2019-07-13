use crate::db::DB;
use duck_family_db_lib::models::*;
use duck_family_api_lib::shop::*;
use duck_family_api_lib::types;
use crate::buildings::BuildingFactory;
use crate::StringErr;


impl DB {
    // TODO: This should be with village/user parameters
    pub fn try_buy_building(&self, typ: types::BuildingType, pos: (usize ,usize)) -> StringErr {
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

    fn building_has_space(&self,  _typ: types::BuildingType, _pos: (usize ,usize)) -> StringErr {
        // TODO Check if bulding slot is empty
        Ok(())
    }
}
