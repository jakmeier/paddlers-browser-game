//! Module for setup code, such as
//!  - Server initialization
//!  - Map generation
//!  - Player creation

mod map_generation;
mod new_player;

use dotenv::dotenv;
use std::env;
use diesel::result::{Error, DatabaseErrorKind};
use paddlers_shared_lib::{
    prelude::*,
    sql_db::run_db_migrations,
};
use crate::db::DB;

pub (crate) fn initialize_new_player_account(db: &DB, uuid: uuid::Uuid) -> Result<(), String> {
    // TODO: Player display name
    let result = db.new_player("Player Display Name (TODO)".to_owned(), uuid);
    if let Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = result {
        println!("Warning: Tried to create player account that already exists");
        Ok(())
    } else {
        result.map_err(|_e| "Player creation failed".to_owned()).map(|_p|())
    }
}

impl DB {

    pub fn db_scripts_by_env(&self) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        if env::var("DATABASE_INIT").is_ok() {
            let server = 1;
            run_db_migrations(self.dbconn())?;
            self.init_map(server);
        }
        if env::var("INSERT_TEST_DATA").is_ok() {
            // self.new_player("Fred Feuerstein".to_owned());
            // self.new_player("Donald Duck".to_owned());
            // self.new_player("Queen Elizabeth".to_owned());
        }
        Ok(())
    }
}
