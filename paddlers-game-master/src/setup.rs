//! Module for setup code, such as
//!  - Server initialization
//!  - Map generation
//!  - Player creation

mod map_generation;
mod new_player;

use dotenv::dotenv;
use std::env;
use paddlers_shared_lib::{
    prelude::*,
    sql_db::run_db_migrations,
};
use crate::db::DB;

impl DB {

    pub fn db_scripts_by_env(&self) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        if env::var("DATABASE_INIT").is_ok() {
            let server = 1;
            run_db_migrations(self.dbconn())?;
            self.init_map(server);
        }
        if env::var("INSERT_TEST_DATA").is_ok() {
            self.new_player("Fred Feuerstein".to_owned());
            self.new_player("Donald Duck".to_owned());
            self.new_player("Queen Elizabeth".to_owned());
        }
        Ok(())
    }
}
