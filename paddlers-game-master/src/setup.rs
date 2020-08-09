//! Module for setup code, such as
//!  - Server initialization
//!  - Map generation
//!  - Player creation

mod map_generation;
mod new_player;

use crate::buildings::BuildingFactory;
use crate::db::DB;
use diesel::result::{DatabaseErrorKind, Error};
use dotenv::dotenv;
use paddlers_shared_lib::test_data::*;
use paddlers_shared_lib::{
    api::PlayerInitData, prelude::*, sql_db::run_db_migrations, story::story_state::StoryState,
};
use std::env;

pub(crate) fn initialize_new_player_account(
    db: &DB,
    uuid: uuid::Uuid,
    info: &PlayerInitData,
) -> Result<(), String> {
    let result = db.new_player(info.display_name.clone(), uuid);
    if let Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _info)) = result {
        println!("Warning: Tried to create player account that already exists");
        Ok(())
    } else {
        result
            .map_err(|_e| "Player creation failed".to_owned())
            .map(|_p| ())
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
            if let Ok(player) = self.new_player(
                TEST_PLAYER_NAME.to_owned(),
                uuid::Uuid::parse_str(TEST_PLAYER_UUID).unwrap(),
            ) {
                let village = self.player_villages(player.key())[0];
                self.add_prophet(village.key());
                self.add_karma(player.key(), 50000).unwrap();
                self.set_story_state(player.key(), StoryState::FirstVisitorWelcomed)?;
                self.insert_temple(village.key());
            }
            for i in 0..ADDITIONAL_PLAYERS {
                let player =
                    self.new_player(format!("Generated_Tester_{}", i), uuid::Uuid::new_v4())?;
                self.set_story_state(player.key(), StoryState::MoreHappyVisitors)?;
            }
        }
        Ok(())
    }
    fn insert_temple(&self, village: VillageKey) {
        let building = BuildingFactory::new(BuildingType::Temple, (4, 2), village);
        self.insert_building(&building);
    }
}
