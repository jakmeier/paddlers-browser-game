use diesel::PgConnection;
use diesel::prelude::*;
use db_lib::sql::GameDB;
use db_lib::models::*;
use db_lib::schema::*;

pub (crate) struct DB (PgConnection);

impl DB {
    pub fn new() -> Self {
        let connection = db_lib::establish_connection();
        DB(connection)
    }

    pub fn delete_unit(&self, unit: &Unit) {
        let result = diesel::delete(unit).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete unit {:?}", unit);
        }
    }

    pub fn delete_attack(&self, atk: &Attack) {
        let result = diesel::delete(atk).execute(self.dbconn());
        if result.is_err() {
            println!("Couldn't delete attack {:?}", atk);
        }
    }

    pub fn insert_unit(&self, u: &NewUnit) -> Unit {
        diesel::insert_into(units::dsl::units)
            .values(u)
            .get_result(self.dbconn())
            .expect("Inserting unit")
    }
    pub fn insert_attack(&self, new_attack: &NewAttack) -> Attack {
        diesel::insert_into(attacks::dsl::attacks)
            .values(new_attack)
            .get_result(self.dbconn())
            .expect("Inserting attack")
    }
    pub fn insert_attack_to_unit(&self, atu: &AttackToUnit) {
        diesel::insert_into(attacks_to_units::dsl::attacks_to_units)
            .values(atu)
            .execute(self.dbconn())
            .expect("Inserting attack to unit");
    }
}

impl GameDB for DB {
    fn dbconn(&self) -> &PgConnection {
        &self.0
    }
}