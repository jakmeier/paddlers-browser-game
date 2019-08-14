use dotenv::dotenv;
use std::env;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use paddlers_shared_lib::{
    prelude::*,
    sql_db::run_db_migrations,
};
pub mod diesel_queries;
pub use diesel_queries::*;
type Manager = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Manager>;
pub (crate) struct DB (r2d2::PooledConnection<Manager>);

impl DB {

    pub fn new_pool() -> Pool {
        let url = paddlers_shared_lib::get_db_url();
        let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    pub fn db_scripts_by_env(&self) -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        if env::var("DATABASE_INIT").is_ok() {
            run_db_migrations(self.dbconn())?;
            self.init_resources();
        }
        if env::var("INSERT_TEST_DATA").is_ok() {
            self.insert_test_data();
        }
        Ok(())
    }

    fn insert_test_data(&self) {
        if self.units(1).len() == 0 {
            self.insert_startkit();
        }
    }

    fn insert_startkit(&self) {
            // Our brave Hero
            let (x,y) = (0,0);
            let unit = NewUnit {
                unit_type: UnitType::Hero,
                x: x,
                y: y,
                color: None,
                hp: 10, 
                speed: 0.5,
                home: 1
            };
            let unit = self.insert_unit(&unit);
            let task = NewTask {
                unit_id: unit.id,
                task_type: TaskType::Idle,
                x: x,
                y: y,
                start_time: None,
            };
            self.insert_task(&task);

            // Some cash
            self.add_resource(ResourceType::Feathers, 50).expect("Adding resources");
            self.add_resource(ResourceType::Sticks, 50).expect("Adding resources");
            self.add_resource(ResourceType::Logs, 50).expect("Adding resources");
    }

}

impl From<&Pool> for DB {
    fn from(pool: &Pool) -> Self {
        DB(pool.get().expect("Couldn't get DB connection"))
    }
}

impl GameDB for DB {
    fn dbconn(&self) -> &PgConnection {
        &self.0
    }
}