use crate::prelude::{VillageKey, HoboKey, WorkerKey, PlayerKey};
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub trait GameDB {
    fn dbconn(&self) -> &PgConnection;

    fn player(&self, player_id: PlayerKey) -> Option<Player> {
        players::table
            .find(player_id.num())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn player_by_uuid(&self, uuid: uuid::Uuid) -> Option<Player> {
        players::table
            .filter(players::uuid.eq(uuid))
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn hobo(&self, hobo_id: i64) -> Option<Hobo> {
        let results = hobos::table
            .filter(hobos::id.eq(hobo_id))
            .get_result::<Hobo>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn hobos(&self, village: VillageKey) -> Vec<Hobo> {
        let results = hobos::table
            .filter(hobos::home.eq(village.num()))
            .limit(500)
            .load::<Hobo>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn worker_priv(&self, worker_id: WorkerKey) -> Option<Worker> {
        let results = workers::table
            .filter(workers::id.eq(worker_id.num()))
            .get_result::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn worker_auth_by_player(&self, worker_id: WorkerKey, player_id: PlayerKey) -> Option<Worker> {
        let results = villages::table
            .inner_join(workers::table)
            .inner_join(players::table)
            .filter(players::id.eq(player_id.num()))
            .filter(workers::id.eq(worker_id.num()))
            .select(workers::all_columns)
            .first::<Worker>(self.dbconn())
            .optional()
            .expect("Error loading data");
        results
    }
    fn workers(&self, village: VillageKey) -> Vec<Worker> {
        let results = workers::table
            .filter(workers::home.eq(village.num()))
            .limit(500)
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn workers_with_job(&self, village: VillageKey, jobs: &[TaskType]) -> Vec<Worker> {
        let results = workers::table
            .inner_join(tasks::table)
            .filter(workers::home.eq(village.num()))
            .filter(tasks::task_type.eq_any(jobs))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .select(workers::all_columns)
            .distinct()
            .load::<Worker>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn count_workers_at_pos_doing_job(&self, village: VillageKey, x: i32, y: i32, job: TaskType ) -> usize {
        workers::table
            .inner_join(tasks::table)
            .filter(tasks::task_type.eq(job))
            .filter(workers::home.eq(village.num()))
            .filter(tasks::x.eq(x))
            .filter(tasks::y.eq(y))
            .select(diesel::dsl::count(workers::id))
            .first::<i64>(self.dbconn())
            .expect("Error loading data") as usize
    }
    fn attacks(&self, village: VillageKey, min_id: Option<i64>) -> Vec<Attack> {
        let results = attacks::table
            .filter(attacks::destination_village_id.eq(village.num()))
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
    fn buildings(&self, village: VillageKey) -> Vec<Building> {
        let results = buildings::table
            .filter(buildings::village_id.eq(village.num()))
            .limit(500)
            .load::<Building>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn find_building_by_coordinates(&self, x: i32, y: i32, village: VillageKey) -> Option<Building> {
        let result = buildings::table
            .filter(buildings::village_id.eq(village.num()))
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
    fn worker_tasks(&self, worker_id: WorkerKey) -> Vec<Task> {
        let results = tasks::table
        .filter(tasks::worker_id.eq(worker_id.num()))
        .limit(500)
        .load::<Task>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn worker_abilities(&self, worker_id: WorkerKey) -> Vec<Ability> {
        let results = abilities::table
        .filter(abilities::worker_id.eq(worker_id.num()))
        .limit(10)
        .load::<Ability>(self.dbconn())
        .expect("Error loading data");
        results
    }
    fn worker_ability(&self, worker_id: WorkerKey, ability_type: AbilityType) -> Option<Ability> {
        abilities::table
        .find((ability_type, worker_id.num()))
        .first(self.dbconn())
        .optional()
        .expect("Error loading data")
    }
    fn past_worker_tasks(&self, worker_id: WorkerKey) -> Vec<Task> {
        let results = tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .filter(tasks::start_time.lt(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .limit(500)
            .load::<Task>(self.dbconn())
            .expect("Error loading data");
        results
    }
    fn earliest_future_task(&self, worker_id: WorkerKey) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
            .filter(tasks::start_time.ge(diesel::dsl::now.at_time_zone("UTC")))
            .order(tasks::start_time.asc())
            .first(self.dbconn())
            .optional()
            .expect("Error loading data")
    }
    fn current_and_next_task(&self, worker_id: WorkerKey) -> (Option<Task>, Option<Task>) {
        let mut results = tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
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
    fn current_task(&self, worker_id: WorkerKey) -> Option<Task> {
        tasks::table
            .filter(tasks::worker_id.eq(worker_id.num()))
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
    fn village(&self, village: VillageKey) -> Option<Village> {
        villages::table
            .find(village.num())
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
    fn player_villages(&self, player_id: PlayerKey) -> Vec<Village> {
        let results = villages::table
            .filter(villages::player_id.eq(player_id.num()))
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
    fn all_player_villages(&self) -> Vec<Village> {
        let results = villages::table
            .filter(villages::player_id.is_not_null())
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
    fn village_owned_by(&self, vid: VillageKey, uuid: uuid::Uuid) -> bool {
        diesel::select(diesel::dsl::exists(
            players::table
                .inner_join(villages::table)
                .filter(players::uuid.eq(uuid))
                .filter(villages::id.eq(vid.num()))
        ))
        .get_result(self.dbconn())
        .expect("Error in look up")
    }
    fn worker_owned_by(&self, wid: WorkerKey, uuid: uuid::Uuid) -> bool {
        diesel::select(diesel::dsl::exists(
            players::table
                .inner_join(villages::table)
                .inner_join(workers::table.on(workers::home.eq(villages::id)))
                .filter(players::uuid.eq(uuid))
                .filter(workers::id.eq(wid.num()))
        ))
        .get_result(self.dbconn())
        .expect("Error in look up")
    }
}