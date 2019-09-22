//! Module for setup code, such as
//!  - Server initialization
//!  - Map generation
//!  - Player creation

mod map_generation;

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
            self.init_resources();
        }
        if env::var("INSERT_TEST_DATA").is_ok() {
            self.insert_test_villages();
            self.init_resources();
            self.insert_test_data();
        }
        Ok(())
    }

    fn insert_test_villages(&self) {
        let required_id = TEST_VILLAGE_ID.num().max(TEST_AI_VILLAGE_ID.num());
        while required_id > self.all_villages().iter().map(|v| v.id).fold(0, |a, b| a.max(b)) {
            self.add_village().unwrap();
        }
    }

    fn insert_test_data(&self) {
        if self.workers(TEST_VILLAGE_ID.num()).len() == 0 {
            self.insert_startkit();
        }
    }

    fn insert_startkit(&self) {
            // Our brave Hero
            let (x,y) = (0,0);
            let worker = NewWorker {
                unit_type: UnitType::Hero,
                x: x,
                y: y,
                color: None,
                speed: 0.5,
                home: TEST_VILLAGE_ID.num(),
                mana: Some(10),
            };
            let worker = self.insert_worker(&worker);
            let task = NewTask {
                worker_id: worker.id,
                task_type: TaskType::Idle,
                x: x,
                y: y,
                start_time: None,
                target_hobo_id: None,
            };
            self.insert_task(&task);
            let work_ability = NewAbility {
                worker_id: worker.id,
                ability_type: AbilityType::Work,
            };
            self.insert_ability(&work_ability);
            let welcome_ability = NewAbility {
                worker_id: worker.id,
                ability_type: AbilityType::Welcome,
            };
            self.insert_ability(&welcome_ability);

            // Some cash
            let vid = TEST_VILLAGE_ID;
            self.add_resource(ResourceType::Feathers, vid, 50).expect("Adding starter kit resources");
            self.add_resource(ResourceType::Sticks, vid, 50).expect("Adding starter kit resources");
            self.add_resource(ResourceType::Logs, vid, 50).expect("Adding starter kit resources");
    }
}
