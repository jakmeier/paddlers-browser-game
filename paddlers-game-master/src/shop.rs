use crate::db::DB;
use paddlers_db_lib::sql::GameDB;
use paddlers_api_lib::shop::*;
use paddlers_api_lib::types;
use paddlers_api_lib::attributes::Attributes;
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

    fn building_has_space(&self,  typ: types::BuildingType, pos: (usize ,usize)) -> StringErr {
        
        // Check conflict with existing building
        let (w,h) = typ.size();
        debug_assert_eq!(w, 1, "Not implemented yet");
        debug_assert_eq!(h, 1, "Not implemented yet");
        let(x0,y0) = (pos.0 as usize, pos.1 as usize);
        // let(x1,y1) = (x0+w, y0+h);
        for other in self.buildings() {
            let typ : types::BuildingType = other.building_type.into();
            let (w,h) = typ.size();
            debug_assert_eq!(w, 1, "Not implemented yet");
            debug_assert_eq!(h, 1, "Not implemented yet");
            let (x,y) = (other.x as usize, other.y as usize);
            if x == x0 && y == y0 
            {
                return Err("No space for building".to_owned());
            }
        }

        // Check conflict with map
        // TODO: Cleaner handling of map shape
        if y0 == 6 {
            return Err("Cannot build here".to_owned());
        }
        Ok(())
    }
}
