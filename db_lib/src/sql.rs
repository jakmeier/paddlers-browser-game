use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub trait GameDB {
    fn dbconn(&self) -> &PgConnection;

    fn units(&self) -> Vec<Unit> {
        let results = units::table
            .limit(500)
            .load::<Unit>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attacks(&self) -> Vec<Attack> {
        let results = attacks::table
            .limit(500)
            .load::<Attack>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_units(&self, atk: &Attack) -> Vec<Unit> {
        let results = attacks_to_units::table
        .inner_join(units::table)
        .filter(attacks_to_units::attack_id.eq(atk.id))
        .select(UNIT_ALL_COLUMNS) 
        .limit(500)
        .load::<Unit>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn buildings(&self) -> Vec<Building> {
        let results = buildings::table
            .limit(500)
            .load::<Building>(self.dbconn())
            .expect("Error loading data");
        results
    }
}