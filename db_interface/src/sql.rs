use crate::DbConn;
use db_lib::models::*;
use db_lib::schema::*;
use diesel::prelude::*;

impl DbConn {
    pub fn units(&self) -> Vec<Unit> {
        let results = units::table
            .limit(500)
            .load::<Unit>(&**self)
            .expect("Error loading data");
        results
    }
    pub fn attacks(&self) -> Vec<Attack> {
        let results = attacks::table
            .limit(500)
            .load::<Attack>(&**self)
            .expect("Error loading data");
        results
    }
    pub fn attack_units(&self, atk: &Attack) -> Vec<Unit> {
        let results = attacks_to_units::table
        .inner_join(units::table)
        .filter(attacks_to_units::attack_id.eq(atk.id))
        .select(UNIT_ALL_COLUMNS) 
        .limit(500)
        .load::<Unit>(&**self)
        .expect("Error loading data");
        results
    }
    pub fn buildings(&self) -> Vec<Building> {
        let results = buildings::table
            .limit(500)
            .load::<Building>(&**self)
            .expect("Error loading data");
        results
    }
}