use chrono::NaiveDateTime;
use crate::prelude::{VillageKey, HoboKey, WorkerKey};
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub trait GameDB {
    fn dbconn(&self) -> &PgConnection;

    fn hobo(&self, hobo_id: i64) -> Option<Hobo> {
        let results = hobos::table
            .filter(hobos::id.eq(hobo_id))
            .get_result::<Hobo>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn hobos(&self, village_id: i64) -> Vec<Hobo> {
        let results = hobos::table
            .filter(hobos::home.eq(village_id))
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn worker(&self, worker_id: i64) -> Option<Worker> {
        let results = workers::table
            .filter(workers::id.eq(worker_id))
            .get_result::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn workers(&self, village_id: i64) -> Vec<Worker> {
        let results = workers::table
            .filter(workers::home.eq(village_id))
            .limit(500)
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn workers_with_job(&self, village_id: i64, jobs: &[TaskType]) -> Vec<Worker> {
        let results = workers::table
            .inner_join(tasks::table)
            .filter(workers::home.eq(village_id))
            .filter(tasks::task_type.eq_any(jobs))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .select(workers::all_columns)
            .distinct()
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn count_workers_at_pos_doing_job(&self, village_id: i64, x: i32, y: i32, job: TaskType ) -> usize {
        workers::table
            .inner_join(tasks::table)
            .filter(tasks::task_type.eq(job))
            .filter(workers::home.eq(village_id))
            .filter(tasks::x.eq(x))
            .filter(tasks::y.eq(y))
            .select(diesel::dsl::count(workers::id))
            .first::<i64>(self.dbconn())
            .expect("Error loading data") as usize
    }
    fn attacks(&self, min_id: Option<i64>) -> Vec<Attack> {
        let results = attacks::table
            .filter(attacks::id.ge(min_id.unwrap_or(0)))
            .limit(500)
            .load::<Attack>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn attack_hobos(&self, atk: &Attack) -> Vec<Hobo> {
        let results = attacks_to_hobos::table
        .inner_join(hobos::table)
        .filter(attacks_to_hobos::attack_id.eq(atk.id))
        .select(hobos::all_columns)
        .limit(500)
        .load::<Hobo>(self.dbconn())
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
    fn find_building_by_coordinates(&self, x: i32, y: i32) -> Option<Building> {
        let result = buildings::table
            .filter(buildings::x.eq(x).and(buildings::y.eq(y)))
            .first::<Building>(self.dbconn())
            .optional()
            .expect("Error loading data");
        result
    }
    fn maybe_resource(&self, r: ResourceType, v: VillageKey) -> Option<i64> {
        resources::table
        .find((r,v.num()))
        .first(self.dbconn())
        .map(|res: Resource| res.amount)
        .optional()
        .expect("Error loading data")
    }
    fn resource(&self, r: ResourceType, v: VillageKey) -> i64 {
        resources::table
        .find((r,v.num()))
        .first(self.dbconn())
        .map(|res: Resource| res.amount)
        .unwrap_or(0)
    }
    fn worker_tasks(&self, worker_id: i64) -> Vec<Task> {
        let results = tasks::table
        .filter(tasks::worker_id.eq(worker_id))
        .limit(500)
        .load::<Task>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn worker_abilities(&self, worker_id: i64) -> Vec<Ability> {
        let results = abilities::table
        .filter(abilities::worker_id.eq(worker_id))
        .limit(10)
        .load::<Ability>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn worker_ability(&self, worker_id: i64, ability_type: AbilityType) -> Option<Ability> {
        abilities::table
        .find((ability_type, worker_id))
        .first(self.dbconn())
        .optional()
        .expect("Error loading data")
    }
    fn past_worker_tasks(&self, worker_id: i64) -> Vec<Task> {
        let results = tasks::table
            .filter(tasks::worker_id.eq(worker_id))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .limit(500)
            .load::<Task>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn earliest_future_task(&self, worker_id: i64) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id))
            .filter(tasks::start_time.ge(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn current_and_next_task(&self, worker_id: i64) -> (Option<Task>, Option<Task>) {
        let mut results = tasks::table
            .filter(tasks::worker_id.eq(worker_id))
            .order(tasks::start_time.asc())
            .limit(2)
            .load(self.dbconn())
            .expect("Error loading data");
        if results.len() == 1 {
            (results.pop(), None)
        } else {
            let next = results.pop();
            let current = results.pop();
            (current, next)
        }
    }
    fn current_task(&self, worker_id: i64) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id))
            .filter(tasks::start_time.le(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn task(&self, task_id: i64) -> Option<Task> {
        tasks::table
            .find(task_id)
            .first(self.dbconn())
            .optional()
            .expect("Error loading task")
    }
    fn streams(&self, low_x: f32, high_x: f32) -> Vec<Stream> {
        let results = streams::table
            .filter(streams::start_x.ge(low_x))
            .filter(streams::start_x.le(high_x))
            .load::<Stream>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn village(&self, village_id: i64) -> Option<Village> {
        villages::table
            .find(village_id)
            .first(self.dbconn())
            .optional()
            .expect("Error loading village")
    }
    fn village_at(&self, x: f32, y: f32) -> Option<Village> {
        villages::table
            .filter(villages::x.ge(x))
            .filter(villages::x.lt(1.0+x))
            .filter(villages::y.ge(y))
            .filter(villages::y.lt(1.0+y))
            .first(self.dbconn())
            .optional()
            .expect("Error looking up village from position")
    }
    fn villages(&self, low_x: f32, high_x: f32) -> Vec<Village> {
        let results = villages::table
            .filter(villages::x.ge(low_x))
            .filter(villages::x.le(high_x))
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn all_villages(&self) -> Vec<Village> {
        let results = villages::table
            .load::<Village>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn effects_on_hobo(&self, hobo: HoboKey) -> Vec<Effect> {
        let results = effects::table
            .filter(effects::hobo_id.eq(hobo.num()))
            .limit(500)
            .load::<Effect>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn last_update(&self, worker: WorkerKey, f: WorkerFlagType) -> Option<NaiveDateTime> {
        worker_flags::table
        .find((worker.num(), f))
        .first(self.dbconn())
        .map(|flag: WorkerFlag| flag.last_update)
        .optional()
        .expect("Error loading data")
    }
}